mod imp;
use crate::types::{PlayerState, Song, SongFav};
use crate::utils::{is_json_valid_and_not_empty, load_data_from_fav};
use gstreamer as gst;
use glib::{subclass::types::ObjectSubclassIsExt};
use gst::glib::object::ObjectExt as GstObjectExt;
use glib::ObjectExt;
use std::time::Duration;
use gst::prelude::*;
glib::wrapper! {
    pub struct PlayerCore(ObjectSubclass<imp::PlayerCoreImp>)
    @extends gtk::Box, gtk::Container, gtk::Widget;
}

impl PlayerCore {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create PlayerCore")
    }
    pub fn with_playlist(playlist: Vec<Song>) -> Self {
        let player = Self::new();
        player.set_playlist(playlist);
        player.load_playlist_fav();
        player
    }
    pub fn get_current_song(&self) -> Song{
        let imp = self.imp();
        let current_index = *imp.current_index.borrow();
        imp.playlist.borrow().get(current_index).unwrap().clone()
    }
    fn load_playlist_fav(&self){
        let imp = self.imp();
        let path = "src/cache/songs_favorities.json";
        *imp.playlist_fav.borrow_mut() = if is_json_valid_and_not_empty(path){
            load_data_from_fav(path).unwrap_or_default()
        }else{
            Vec::new()
        }
    }
    pub fn get_playlist_fav(&self)->Vec<SongFav>{
        let imp = self.imp();
        imp.playlist_fav.borrow().clone()
    }
    pub fn set_playlist(&self, playlist: Vec<Song>) {
        let imp = self.imp();
        *imp.playlist.borrow_mut() = playlist;
    }

    pub fn get_playlist(&self) -> Vec<Song> {
        self.imp().playlist.borrow().clone()
    }
    
    pub fn set_volume(&self,vol:f64){
        let imp = self.imp();
        let playbin_ref = imp.playbin.borrow();
        if let Some(playbin) = playbin_ref.as_ref() {
            playbin.set_property("volume",&vol);
        }
    }
 

    pub fn mute(&self){
        let imp = self.imp();
        let playbin_ref = imp.playbin.borrow();
        if let Some(playbin) = playbin_ref.as_ref(){
            playbin.set_property("mute", &true);
        }
        *imp.muted.borrow_mut() = true;
    }
    pub fn unmute(&self){
        let imp = self.imp();
        let playbin_ref = imp.playbin.borrow();
        if let Some(playbin) = playbin_ref.as_ref() {
            playbin.set_property("mute", &false); 
            *imp.muted.borrow_mut() = true;
        }
        *imp.muted.borrow_mut() = false;
    }
    pub fn is_mute(&self) -> bool{
        let imp = self.imp();
        *imp.muted.borrow()
    }
    pub fn get_current_duration(&self) -> Option<f64>{
        let imp = self.imp();
        let current_idx = *imp.current_index.borrow();
        imp.playlist.borrow()
            .get(current_idx)
            .and_then(|song| {
                Some(song.duration_total_sec)
            })
    }
    pub fn stop_progress_update(&self){
        let imp = self.imp();
        if *imp.progress_update_id.borrow() == None{ return;}

        if let Some(id) = self.imp().progress_update_id.borrow_mut().take(){
            id.remove();
        }
    }
    fn start_progress_updater(&self){
        let self_weak = self.downgrade();

        let imp = self.imp();
        let id = glib::timeout_add_local(Duration::from_millis(100),move || {
            if let Some(self_) = self_weak.upgrade() {
                let playbin_rc = self_.imp().playbin.clone();

                if let Some(playbin) = playbin_rc.borrow().as_ref() {
                    let position_clocktime = playbin
                        .query_position::<gst::ClockTime>()
                        .unwrap_or(gst::ClockTime::ZERO);
                    let position = position_clocktime.seconds() as f64;
                    let duration = self_.get_current_duration().unwrap();
                    let progress = position/duration;
                    self_.emit_by_name::<()>("position-changed", &[&progress]);
                }
                glib::Continue(true) 
            } else {
                glib::Continue(false) 
            }
        });
        *imp.progress_update_id.borrow_mut() = Some(id);
        *imp.is_updating.borrow_mut() = false;
    }

   pub fn skip_forward(&self, offset_seconds: u64) {
    let playbin_ref = self.imp().playbin.borrow();
    if let Some(playbin) = playbin_ref.as_ref() {
        let new_pos = gst::ClockTime::from_seconds(offset_seconds);
        let current_state = playbin.current_state();
        if current_state == gst::State::Playing || current_state == gst::State::Paused {
            match playbin.seek_simple(
                gst::SeekFlags::FLUSH | gst::SeekFlags::ACCURATE,
                new_pos,
            ) {
                Ok(_) => {
                    self.start_progress_updater();
                }
                Err(e) => {
                    eprintln!("Error al hacer seek: {}", e);
                }
            }
        } else {
            eprintln!(
                "No se puede hacer seek: estado actual = {:?}",
                current_state
            );
        }
    }
}

    pub fn play(&self) {
        let imp = self.imp();
        let current_index = imp.current_index.borrow_mut();
        let mut flag = imp.is_another_index.borrow_mut();
        if let Some(song) = imp.playlist.borrow().get(*current_index) {
            let uri = format!("file://{}", song.path_file);
            if let Some(playbin) = imp.playbin.borrow_mut().as_mut() {
                if *flag{
                    playbin.set_state(gst::State::Null).ok();
                    *flag = false;
                } 
                playbin.set_property("uri", &uri);
                playbin.set_state(gst::State::Playing).unwrap();
                imp.state.replace(PlayerState::Playing);
                self.set_state(PlayerState::Playing);
                self.emit_by_name::<()>("info-changed", &[&song]);
                self.start_progress_updater();
            }
        }
    }


    pub fn pause(&self) {
        let imp = self.imp();
        let playbin_ref = imp.playbin.borrow();
        if let Some(playbin) = playbin_ref.as_ref() {
            playbin.set_state(gst::State::Paused).ok();
            imp.state.replace(PlayerState::Paused);
            self.set_state(PlayerState::Paused);
        }        

    }
    pub fn stop(&self) {
        let imp = self.imp();
        let playbin_ref = imp.playbin.borrow();
        if let Some(playbin) = playbin_ref.as_ref() {
            playbin.set_state(gst::State::VoidPending).ok();
            imp.state.replace(PlayerState::Paused);
            self.set_state(PlayerState::Stopped);
        }   
    }
    
    pub fn next(&self){
        let imp = self.imp();
        {
            let mut idx = imp.current_index.borrow_mut();
            let mut is_click = imp.is_another_index.borrow_mut();
            let playlist_len = imp.playlist.borrow().len();
            if *idx+1 < playlist_len {
                *idx += 1;
            }else{
                *idx = 0;
            }
            *is_click = true;
        }
        self.play();      
    }
    pub fn prev(&self){
        let imp = self.imp();
        {
            let mut idx = imp.current_index.borrow_mut();
            let mut is_click = imp.is_another_index.borrow_mut();
            let playlist_len = imp.playlist.borrow().len();

            if playlist_len == 0 {
                return;
            }
            if *idx == 0 {
                *idx = playlist_len - 1;
            } else {
                *idx -= 1;
            }
            *is_click = true;
        }
        self.play();
    }
    pub fn add_remove_song_fav(&self,idx:usize,flag:bool)->bool{
        let imp = self.imp();
        let playlist = imp.playlist.borrow();
        if let Some(song) = playlist.get(idx) {
            let song = song.clone();
            let path_file = song.path_file.clone(); 
            let s :SongFav = SongFav { 
                id: idx,
                title: song.title,
                artist: song.artist,
                path_file: song.path_file,
                duration_sec: song.duration_sec,
                duration_min: song.duration_min,
                duration_total_sec: song.duration_total_sec,
                is_favorite: flag
            }; 
            if flag {
                imp.playlist_fav.borrow_mut().push(s.clone());
            } else {
                imp.playlist_fav.borrow_mut().retain(|sf| sf.path_file != path_file);
            }; 
        }
        self.emit_by_name::<()>("add-remove-favs", &[&flag]);
        return flag;
    }
    fn set_state(&self, new_state: PlayerState) {
        *self.imp().state.borrow_mut() = new_state;
        let state_int = match new_state {
            PlayerState::Playing => 0,
            PlayerState::Paused => 1,
            PlayerState::Stopped => 2,
        };
        self.emit_by_name::<()>("state-changed", &[&state_int]);
    }
}
