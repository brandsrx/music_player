use crate::loader::load_cover_pixbuf;
use crate::utils::{save_data_to_fav};
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::prelude::*;
use gtk::{Box, Button, Scale, Orientation, Label, Image};
use std::rc::Rc;
use glib::clone;
use std::cell::RefCell;
use crate::types::{format_duration_enhanced, PlayerState, Song};
use crate::kernel::PlayerCore;
use glib::signal::Inhibit;
use gdk_pixbuf::Pixbuf;
#[derive(Clone)]
pub struct PackInfoSong {
    pub song_title: Label,
    pub song_artist: Label,
    pub album_image: Image,
    pub favorite:Button
}


fn change_button_icon(button: gtk::Button, icon_name: &str) {
    let icon = Image::from_icon_name(Some(icon_name), gtk::IconSize::Button);
    button.set_image(Some(&icon));
}
fn change_info(
    widgets: PackInfoSong,
    new_song: Song,
    total_time: Label,
    _player: PlayerCore, // ya no se necesita si no haces lógica interna
    current_song: Rc<RefCell<Song>>,
) {
    // Actualiza el estado compartido
    *current_song.borrow_mut() = new_song.clone();

    // Formato de duración
    let format_time = format_duration_enhanced(new_song.duration_min, new_song.duration_sec);
    widgets.song_title.set_text(&new_song.title);
    widgets.song_artist.set_text(&new_song.artist);
    total_time.set_text(&format_time);

    // Imagen del álbum
    if let Some(pixbuf) = load_cover_pixbuf(&new_song.path_file, 100, 100) {
        widgets.album_image.set_from_pixbuf(Some(&pixbuf));
    }

    println!("{}",new_song.is_favorite );
    // Cambiar imagen e ícono de favorito
    let (tooltip, icon_path) = if new_song.is_favorite {
        ("Quitar de favoritos", "src/icons/unfavorite.png")
    } else {
        ("Añadir a favoritos", "src/icons/favorite.png")
    };

    widgets.favorite.set_tooltip_text(Some(tooltip));    
    let pixbuf_icon = gdk_pixbuf::Pixbuf::from_file_at_size(icon_path, 20, 20).unwrap();
    let image = gtk::Image::from_pixbuf(Some(&pixbuf_icon));
    widgets.favorite.set_image(Some(&image));

}

pub fn first_row() -> (Box, PackInfoSong,Rc<RefCell<Song>>) {
    // Estado global o compartido
    let current_song = Rc::new(RefCell::new(Song::default()));
    let row = Box::new(Orientation::Horizontal, 15);

    row.style_context().add_class("track-info-container");
    row.set_margin_start(100);
    row.set_margin_top(25);
    row.set_margin_bottom(25);
    // Album artwork with rounded corners
    let album_container = Box::new(Orientation::Horizontal, 0);
    album_container.style_context().add_class("album-artwork-container");
    
    let album_image = Image::from_icon_name(Some("audio-x-generic"), gtk::IconSize::Dialog);
    album_image.set_size_request(100, 100);
    album_image.style_context().add_class("album-artwork");
    album_container.pack_start(&album_image, false, false, 0);

    // Song information with better typography
    let song_info_box = Box::new(Orientation::Vertical, 4);
    song_info_box.set_hexpand(true);
    song_info_box.set_valign(gtk::Align::Center);
    song_info_box.style_context().add_class("song-info-box");
    
    let song_title = Label::new(Some("Sin canción seleccionada"));
    song_title.set_halign(gtk::Align::Start);
    song_title.set_ellipsize(gtk::pango::EllipsizeMode::End);
    song_title.set_max_width_chars(40);
    song_title.style_context().add_class("song-title");
    
    let song_artist = Label::new(Some("Artista desconocido"));
    song_artist.set_halign(gtk::Align::Start);
    song_artist.set_ellipsize(gtk::pango::EllipsizeMode::End);
    song_artist.set_max_width_chars(40);
    song_artist.style_context().add_class("song-artist");

    // Favorite button with heart icon
    let favorite_button = gtk::Button::new();
    let pixbuf_unfav = Rc::new(Pixbuf::from_file_at_size("src/icons/unfavorite.png",20,20).unwrap());
    let favorite_icon = Image::from_pixbuf(Some(&pixbuf_unfav)); // Ícono vacío inicial
   
    favorite_button.set_child(Some(&favorite_icon));
    favorite_button.set_relief(gtk::ReliefStyle::None);
    favorite_button.set_size_request(20, 20);      
    favorite_button.set_hexpand(false);               
    favorite_button.set_halign(gtk::Align::Start);    
    favorite_button.style_context().add_class("heart-button");
    favorite_button.set_relief(gtk::ReliefStyle::None);  

    song_info_box.pack_start(&song_title, false, false, 0);
    song_info_box.pack_start(&song_artist, false, false, 0);
    song_info_box.pack_start(&favorite_button, false, false, 5);

    row.pack_start(&album_container, false, false, 0);
    row.pack_start(&song_info_box, true, true, 0);

    
    let widgets = PackInfoSong {
        song_title: song_title.clone(),
        song_artist: song_artist.clone(),
        album_image: album_image.clone(),
        favorite:favorite_button.clone(),
    };

    (row, widgets,current_song)
}

pub fn second_row(
    player: Rc<RefCell<PlayerCore>>,
    package_info_widgets: PackInfoSong,
    current_song:Rc<RefCell<Song>>
) -> Box {
    let row = Box::new(Orientation::Horizontal, 0);
    row.style_context().add_class("controls-container");

    // Media controls with modern styling
    let control_box = Box::new(Orientation::Horizontal, 12);
    control_box.set_halign(gtk::Align::Center);
    control_box.style_context().add_class("media-controls");

    let prev_button = Button::from_icon_name(Some("media-skip-backward"), gtk::IconSize::Button);
    let play_button = Button::from_icon_name(Some("media-playback-start"), gtk::IconSize::Button);
    let next_button = Button::from_icon_name(Some("media-skip-forward"), gtk::IconSize::Button);

    // Style control buttons
    prev_button.style_context().add_class("control-button");
    prev_button.style_context().add_class("secondary-control");
    prev_button.set_size_request(15, 15);

    play_button.style_context().add_class("control-button");
    play_button.style_context().add_class("primary-control");
    play_button.set_size_request(15, 15);

    next_button.style_context().add_class("control-button");
    next_button.style_context().add_class("secondary-control");
    next_button.set_size_request(15, 15);

    control_box.pack_start(&prev_button, false, false, 0);
    control_box.pack_start(&play_button, false, false, 0);
    control_box.pack_start(&next_button, false, false, 0);

    // Progress section with time labels
    let progress_container = Box::new(Orientation::Vertical, 8);
    progress_container.set_hexpand(true);
    progress_container.style_context().add_class("progress-container");

    let progress_box = Box::new(Orientation::Horizontal, 12);
    
    let current_time = Label::new(Some("0:00"));
    current_time.style_context().add_class("time-label");
    current_time.set_size_request(15, -1);
    current_time.set_margin_start(15);

    let progress_scale = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.01);
    progress_scale.set_hexpand(true);
    progress_scale.set_value(0.0);
    progress_scale.set_draw_value(false);
    progress_scale.style_context().add_class("progress-scale");

    let total_time = Label::new(Some("0:00"));
    total_time.style_context().add_class("time-label");
    total_time.set_size_request(15, -1);
    total_time.set_margin_end(15);

    progress_box.pack_start(&current_time, false, false, 0);
    progress_box.pack_start(&progress_scale, true, true, 0);
    progress_box.pack_start(&total_time, false, false, 0);

    progress_container.pack_start(&progress_box, false, false, 0);

    // Volume controls with elegant design
    let volume_container = Box::new(Orientation::Horizontal, 8);
    volume_container.set_halign(gtk::Align::End);
    volume_container.style_context().add_class("volume-container");

    let volume_button = Button::from_icon_name(Some("audio-volume-high"), gtk::IconSize::Button);
    volume_button.style_context().add_class("control-button");
    volume_button.style_context().add_class("volume-button");
    volume_button.set_size_request(15, 15);
    for button in &[&prev_button, &play_button, &next_button,&volume_button] {
        button.set_relief(gtk::ReliefStyle::None);  // Quita el efecto 3D
        button.set_focus_on_click(false);          // Evita el resaltado al hacer clic
        button.style_context().add_class("flat-button"); // Clase CSS personalizada
    }
    let volume_scale = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.01);
    volume_scale.set_size_request(100, -1);
    volume_scale.set_draw_value(false);
    volume_scale.style_context().add_class("volume-scale");
    
    {
        let player_ref = player.borrow();
        let imp = player_ref.imp();
        let volume = *imp.volume.borrow();
        volume_scale.set_value(volume as f64);
    }

    volume_container.pack_start(&volume_button, false, false, 0);
    volume_container.pack_start(&volume_scale, false, false, 0);

    // Event handlers
    {
        let player_clone = player.clone();
        play_button.connect_clicked(move |btn| {
            let state = player_clone.borrow().imp().state.borrow().clone();

            match state {
                PlayerState::Playing => {
                    player_clone.borrow().pause();
                    change_button_icon(btn.clone(), "media-playback-start");
                }
                PlayerState::Paused | PlayerState::Stopped => {
                    player_clone.borrow().play();
                    change_button_icon(btn.clone(), "media-playback-pause");
                }
            }
        });
    }

    {
        let player_clone = player.clone();

        prev_button.connect_clicked(move |_| {
            let player = player_clone.borrow();
            player.prev();
        });
    }

    {
        let player_clone = player.clone();
        next_button.connect_clicked(move |_| {
            let player = player_clone.borrow();
            player.next();
        });
    }

    {
        let player_clone = player.clone();
        volume_button.connect_clicked(move |btn| {
            let player = player_clone.borrow();

            if player.is_mute() {
                player.unmute();
                change_button_icon(btn.clone(), "audio-volume-high");
            } else {
                player.mute();
                change_button_icon(btn.clone(), "audio-volume-muted");
            }
        });
    }

    {
        progress_scale.connect_change_value(
            clone!(@weak player => @default-return Inhibit(false), move |_, _, _| {
                let player_ref = player.borrow();
                *player_ref.imp().is_updating.borrow_mut() = true;
                player.borrow().stop_progress_update();
                Inhibit(false)
            })
        );
    }

    {
        let current_time = current_time.clone();
        progress_scale.connect_value_changed(
            clone!(@weak player => move |new_value| {
                let player_ref = player.borrow();
                if !*player_ref.imp().is_updating.borrow(){
                    return;
                }
                if let Some(duration) = player.borrow().get_current_duration() {
                    let seek_pos = (new_value.value() * duration) as u64;
                    let new_minutes = (seek_pos/60) as f64;
                    let new_seconds = (new_minutes*60.0) - seek_pos as f64 ; 
                    current_time.set_text(&format_duration_enhanced(new_minutes, new_seconds.abs()));
                    player.borrow().skip_forward(seek_pos);
                }
            })
        );
    }

    {
        let player_clone = player.clone();
        volume_scale.connect_value_changed(move |scale| {
            let vol = scale.value();
            player_clone.borrow().set_volume(vol);
        });
    }

    // Signal handlers
    {
        let player: &PlayerCore = &player.borrow();
        player.connect_local("state-changed", false, move |values| {
            let state_value = values[1].get::<i32>().expect("msg");
            
            match state_value {
                0 => {
                    change_button_icon(play_button.clone(), "media-playback-pause");
                }
                1 => {
                    change_button_icon(play_button.clone(), "media-playback-start");
                }
                2 => println!("Stopped"),
                _ => {
                    eprintln!("Invalid PlayerState value: {}", state_value);
                }
            }
            None
        });
    }

    {
        let player: &PlayerCore = &player.borrow();
        let current_time = current_time.clone();
        player.connect_local("position-changed", false, move |values| {
            let state_value = values[1].get::<f64>().expect("msg");
            let player = values[0].get::<PlayerCore>().unwrap();

            if !player.imp().is_updating.borrow().clone() {
                if let Some(duration) = player.get_current_duration() {
                    let current_position = state_value*duration;
                    let new_minutes:f64 = current_position/60.0;
                    let new_seconds:f64 = (new_minutes*60.0) - current_position; 
                    current_time.set_text(&format_duration_enhanced(new_minutes, new_seconds.abs()));
                }
                progress_scale.set_value(state_value);
            }
            None
        });
    }

    {
        let player: &PlayerCore = &player.borrow();
        let total_time = total_time.clone();
        let widgets = package_info_widgets.clone();
        player.connect_local("info-changed", false, move |values| {
            let song = values[1].get::<Song>().expect("msg");
            let player = values[0].get::<PlayerCore>().expect("msg");
            change_info(widgets.clone(), song.clone(),total_time.clone(),player,current_song.clone());

            None
        });
    }
    row.pack_start(&control_box, false, false, 0);
    row.pack_start(&progress_container, true, true, 0);
    row.pack_start(&volume_container, false, false, 0);
    row
}

pub fn create_control_bar(player: Rc<RefCell<PlayerCore>>) -> Box {
    let main_container = Box::new(Orientation::Vertical, 0);
    main_container.set_height_request(150);
    main_container.style_context().add_class("player-control-bar");

    // Add subtle separator at top
    let separator = gtk::Separator::new(Orientation::Horizontal);
    separator.style_context().add_class("control-separator");

    // Track info section
    let (info_box, pack_info_widgets,current_song) = first_row();
    
    // Controls section
    let controls = second_row(player.clone(), pack_info_widgets.clone(),current_song.clone());
    let pixbuf_unfav = Rc::new(Pixbuf::from_file_at_size("src/icons/unfavorite.png", 20, 20).unwrap());
    let pixbuf_fav = Rc::new(Pixbuf::from_file_at_size("src/icons/favorite.png", 20, 20).unwrap());
    let button = pack_info_widgets.favorite.clone();

    let player: &PlayerCore = &player.borrow();
    button.connect_clicked(clone!(@strong player, @strong current_song, @strong pixbuf_unfav, @strong pixbuf_fav => move |button| {
        let imp = player.imp();
        let current_idx = *imp.current_index.borrow();
        let is_favorite: bool;

        {
        let mut playlist = imp.playlist.borrow_mut();
        let song = &mut playlist[current_idx];

        song.is_favorite = !song.is_favorite;
        current_song.borrow_mut().is_favorite = song.is_favorite;
         is_favorite = song.is_favorite;
        }
        let image = if player.add_remove_song_fav(current_idx, is_favorite) {
            Image::from_pixbuf(Some(&pixbuf_unfav))
        } else {
            Image::from_pixbuf(Some(&pixbuf_fav))
        }; 
        
        save_data_to_fav(&imp.playlist_fav.borrow(), "src/cache/songs_favorities.json").ok();
        button.set_image(Some(&image));
        button.set_tooltip_text(Some(
            if is_favorite { "Quitar de favoritos" } else { "Añadir a favoritos" }
        ));
    }));

    main_container.pack_start(&separator, false, false, 0);
    main_container.pack_start(&info_box, false, false, 0);
    main_container.pack_start(&controls, false, false, 0);
    
    main_container
}