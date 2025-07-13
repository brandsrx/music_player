use gtk::Button;
use serde::{Serialize, Deserialize};



#[derive(Debug,Default, Clone,Serialize,Deserialize, glib::Boxed,PartialEq)]
#[boxed_type(name = "Song")]
pub struct Song{
    pub title : String,
    pub artist : String,
    pub path_file : String,
    pub duration_sec : f64,
    pub duration_min : f64,
    pub duration_total_sec:f64,
    pub is_favorite : bool
}

#[derive(Debug,Default, Clone,Serialize,Deserialize, glib::Boxed,PartialEq)]
#[boxed_type(name = "Song")]
pub struct SongFav{
    pub id : usize,
    pub title : String,
    pub artist : String,
    pub path_file : String,
    pub duration_sec : f64,
    pub duration_min : f64,
    pub duration_total_sec:f64,
    pub is_favorite : bool
}
impl From<SongFav> for Song {
    fn from(song_fav: SongFav) -> Self {
        Song {
            title: song_fav.title,
            artist: song_fav.artist,
            path_file: song_fav.path_file,
            duration_sec: song_fav.duration_sec,
            duration_min: song_fav.duration_min,
            duration_total_sec: song_fav.duration_total_sec,
            is_favorite: song_fav.is_favorite,
        }
    }
}


#[derive(PartialEq, Clone, Copy, Default, Debug)]
pub enum PlayerState {
    #[default]
    Playing,
    Paused,
    Stopped,
}

pub struct SidebarButtons {
    pub btn_playlist: Button,
    pub btn_favorites: Button,
    pub btn_blioteca: Button,
    pub btn_settings: Button,
    pub btn_exit: Button,
}

pub fn format_duration_enhanced(minutes: f64, seconds: f64) -> String {
    let total_seconds = (minutes * 60.0 + seconds) as u32;
    let mins = total_seconds / 60;
    let secs = total_seconds % 60;
    if mins < 10 {
        return format!("0{}:{:02}",mins, secs);
    }
    if mins >= 60 {
        let hours = mins / 60;
        let remaining_mins = mins % 60;
        format!("{}:{:02}:{:02}", hours, remaining_mins, secs)
    } else {
        format!("{}:{:02}", mins, secs)
    }
}