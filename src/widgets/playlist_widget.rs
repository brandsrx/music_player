use crate::types::{Song, PlayerState};
use crate::loader::load_cover_pixbuf;
use crate::kernel::PlayerCore;
use glib::subclass::types::ObjectSubclassIsExt;
use gtk::prelude::*;
use gtk::{Box, Label, Orientation, EventBox, Separator, Image};
use std::rc::Rc;
use std::cell::RefCell;
use glib::source::timeout_add_local;
use std::time::Duration;
use gdk::EventMask;

pub fn create_song_widget(song: &Song, index: usize, player: Rc<RefCell<PlayerCore>>) -> Box {
    let main_container = Box::new(Orientation::Vertical, 0);
    main_container.style_context().add_class("playlist-item-container");

    // EventBox principal con hover mejorado
    let event_box = EventBox::new();
    event_box.set_above_child(true);
    event_box.set_visible_window(true);
    event_box.set_events(
        EventMask::ENTER_NOTIFY_MASK
            | EventMask::LEAVE_NOTIFY_MASK
            | EventMask::BUTTON_PRESS_MASK
            | EventMask::POINTER_MOTION_MASK,
    );
    event_box.style_context().add_class("playlist-item-eventbox");

    // Contenedor principal con padding mejorado
    let row_box = Box::new(Orientation::Horizontal, 16);
    row_box.set_size_request(-1, 72);
    row_box.set_margin_start(16);
    row_box.set_margin_end(16);
    row_box.set_margin_top(4);
    row_box.set_margin_bottom(4);
    row_box.style_context().add_class("playlist-item-row");

    // Sección de número/control de reproducción
    let track_control_section = Box::new(Orientation::Horizontal, 0);
    track_control_section.set_size_request(48, -1);
    track_control_section.set_halign(gtk::Align::Center);
    track_control_section.set_valign(gtk::Align::Center);
    track_control_section.style_context().add_class("track-control-section");

    // Número de pista
    let track_number = Label::new(Some(&format!("{}", index + 1)));
    track_number.set_halign(gtk::Align::Center);
    track_number.set_valign(gtk::Align::Center);
    track_number.style_context().add_class("track-number");

    // Ecualizador minimalista mejorado
    let equalizer = gtk::DrawingArea::new();
    equalizer.set_size_request(24, 18);
    equalizer.set_halign(gtk::Align::Center);
    equalizer.set_valign(gtk::Align::Center);
    equalizer.set_visible(false);
    equalizer.style_context().add_class("equalizer-widget");

    track_control_section.pack_start(&track_number, true, true, 0);
    track_control_section.pack_start(&equalizer, true, true, 0);

    // Cover del álbum con esquinas redondeadas
    let album_cover_container = Box::new(Orientation::Horizontal, 0);
    album_cover_container.set_size_request(56, 56);
    album_cover_container.set_halign(gtk::Align::Center);
    album_cover_container.set_valign(gtk::Align::Center);
    album_cover_container.style_context().add_class("album-cover-container");

    let album_image = Image::from_icon_name(Some("audio-x-generic"), gtk::IconSize::Dialog);
    album_image.style_context().add_class("album-cover-image");

    // Cargar cover personalizado si existe
    let path_file = song.path_file.clone();
    if let Some(pixbuf) = load_cover_pixbuf(&path_file, 56, 56) {
        album_image.set_from_pixbuf(Some(&pixbuf));
    }
    
    album_cover_container.pack_start(&album_image, true, true, 0);

    // Información de la canción mejorada
    let song_info_section = Box::new(Orientation::Vertical, 6);
    song_info_section.set_hexpand(true);
    song_info_section.set_valign(gtk::Align::Center);
    song_info_section.set_margin_start(4);
    song_info_section.style_context().add_class("song-info-section");

    // Título con truncado inteligente
    let title_text = if song.title.len() > 48 {
        format!("{}...", &song.title[..45])
    } else {
        song.title.clone()
    };

    let title_label = Label::new(Some(&title_text));
    title_label.set_halign(gtk::Align::Start);
    title_label.set_ellipsize(gtk::pango::EllipsizeMode::End);
    title_label.set_max_width_chars(50);
    title_label.style_context().add_class("song-title");

    // Artista mejorado
    let artist_text = if song.artist.len() > 52 {
        format!("{}...", &song.artist[..49])
    } else {
        song.artist.clone()
    };

    let artist_label = Label::new(Some(&artist_text));
    artist_label.set_halign(gtk::Align::Start);
    artist_label.set_ellipsize(gtk::pango::EllipsizeMode::End);
    artist_label.set_max_width_chars(55);
    artist_label.style_context().add_class("song-artist");

    song_info_section.pack_start(&title_label, false, false, 0);
    song_info_section.pack_start(&artist_label, false, false, 0);

    // Duración con mejor formato
    let duration_text = format_duration_enhanced(song.duration_min, song.duration_sec);
    let duration_label = Label::new(Some(&duration_text));
    duration_label.set_halign(gtk::Align::End);
    duration_label.set_valign(gtk::Align::Center);
    duration_label.set_margin_end(8);
    duration_label.style_context().add_class("song-duration");

    // Estados mejorados para animaciones
    let is_playing = Rc::new(RefCell::new(false));
    let is_hovered = Rc::new(RefCell::new(false));
    let animation_progress = Rc::new(RefCell::new(0.0));
    let equalizer_bars = Rc::new(RefCell::new(vec![0.2, 0.8, 0.4, 0.9, 0.3, 0.6]));

    // Dibujo del ecualizador mejorado
    {
        let is_playing = is_playing.clone();
        let equalizer_bars = equalizer_bars.clone();
        let animation_progress = animation_progress.clone();

        equalizer.connect_draw(move |_, cr| {
            let widget_width = 24.0;
            let widget_height = 18.0;
            let playing = *is_playing.borrow();

            if playing {
                let bar_count = 6;
                let bar_width = 2.5;
                let bar_spacing = 1.0;
                let total_width = (bar_count as f64 - 1.0) * (bar_width + bar_spacing);
                let start_x = (widget_width - total_width) / 2.0;

                let bars = equalizer_bars.borrow();
                let animation = *animation_progress.borrow();

                for (i, &base_height) in bars.iter().enumerate() {
                    let x = start_x + i as f64 * (bar_width + bar_spacing);
                    let wave_offset = animation + i as f64 * 0.8;
                    let animated_height = base_height * (1.0 + 0.5 * wave_offset.sin());
                    let bar_height = widget_height * animated_height.clamp(0.1, 0.95);
                    let y = widget_height - bar_height;

                    // Gradiente de color para las barras
                    let intensity = animated_height;
                    cr.set_source_rgba(
                        0.0 + intensity * 0.3,
                        0.8 + intensity * 0.2,
                        0.2 + intensity * 0.3,
                        0.9
                    );

                    // Barras con esquinas redondeadas simuladas
                    cr.rectangle(x, y, bar_width, bar_height);
                    cr.fill().unwrap();
                }
            }

            gtk::Inhibit(false)
        });
    }

    // Empaquetado con mejor espaciado
    row_box.pack_start(&track_control_section, false, false, 0);
    row_box.pack_start(&album_cover_container, false, false, 0);
    row_box.pack_start(&song_info_section, true, true, 0);
    row_box.pack_start(&duration_label, false, false, 0);

    event_box.add(&row_box);

    // Eventos de hover mejorados
    event_box.connect_enter_notify_event({
        let track_number = track_number.clone();
        let is_hovered = is_hovered.clone();
        let player = player.clone();
        let row_box = row_box.clone();
        
        move |_, _| {
            *is_hovered.borrow_mut() = true;
            row_box.style_context().add_class("playlist-item-hovered");

            let player_ref = player.borrow();
            let imp = player_ref.imp();
            let current_idx = *imp.current_index.borrow();
            let state = imp.state.borrow().clone();

            if current_idx == index {
                match state {
                    PlayerState::Playing => track_number.set_text("⏸"),
                    PlayerState::Paused | PlayerState::Stopped => track_number.set_text("▶"),
                }
            } else {
                track_number.set_text("▶");
            }

            gtk::Inhibit(false)
        }
    });

    event_box.connect_leave_notify_event({
        let track_number = track_number.clone();
        let is_hovered = is_hovered.clone();
        let player = player.clone();
        let row_box = row_box.clone();
        
        move |_, _| {
            *is_hovered.borrow_mut() = false;
            row_box.style_context().remove_class("playlist-item-hovered");

            let player_ref = player.borrow();
            let imp = player_ref.imp();
            let current_idx = *imp.current_index.borrow();
            let state = imp.state.borrow().clone();

            if current_idx == index {
                match state {
                    PlayerState::Playing => track_number.set_text("♪"),
                    PlayerState::Paused | PlayerState::Stopped => track_number.set_text("⏸"),
                }
            } else {
                track_number.set_text(&format!("{}", index + 1));
            }

            gtk::Inhibit(false)
        }
    });

    // Loop de animación principal mejorado
    {
        let is_playing = is_playing.clone();
        let animation_progress = animation_progress.clone();
        let track_number = track_number.clone();
        let title_label = title_label.clone();
        let artist_label = artist_label.clone();
        let equalizer = equalizer.clone();
        let row_box = row_box.clone();
        let player = player.clone();
        let equalizer_bars = equalizer_bars.clone();

        timeout_add_local(Duration::from_millis(80), move || {
            let player_ref = player.borrow();
            let imp = player_ref.imp();
            let current_idx = *imp.current_index.borrow();
            let state = imp.state.borrow().clone();

            let is_current = current_idx == index;
            let is_playing_now = is_current && state == PlayerState::Playing;

            *is_playing.borrow_mut() = is_playing_now;

            // Actualizar clases CSS
            if is_current {
                row_box.style_context().add_class("playlist-item-current");
                title_label.style_context().add_class("song-title-current");
                artist_label.style_context().add_class("song-artist-current");
                track_number.style_context().add_class("track-number-current");

                if is_playing_now {
                    row_box.style_context().add_class("playlist-item-playing");
                    equalizer.set_visible(true);
                    track_number.set_visible(false);

                    if !*is_hovered.borrow() {
                        track_number.set_text("♪");
                    }
                } else {
                    row_box.style_context().remove_class("playlist-item-playing");
                    equalizer.set_visible(false);
                    track_number.set_visible(true);

                    if !*is_hovered.borrow() {
                        track_number.set_text("⏸");
                    }
                }
            } else {
                row_box.style_context().remove_class("playlist-item-current");
                row_box.style_context().remove_class("playlist-item-playing");
                title_label.style_context().remove_class("song-title-current");
                artist_label.style_context().remove_class("song-artist-current");
                track_number.style_context().remove_class("track-number-current");

                equalizer.set_visible(false);
                track_number.set_visible(true);

                if !*is_hovered.borrow() {
                    track_number.set_text(&format!("{}", index + 1));
                }
            }

            // Animación fluida del ecualizador
            if is_playing_now {
                let mut progress = animation_progress.borrow_mut();
                *progress += 0.18;
                if *progress > 2.0 * std::f64::consts::PI {
                    *progress = 0.0;
                }

                let mut bars = equalizer_bars.borrow_mut();
                for (i, height) in bars.iter_mut().enumerate() {
                    let noise = (fastrand::f64() - 0.5) * 0.12;
                    let wave = (*progress + i as f64 * 0.7).sin() * 0.15;
                    *height = (*height + noise + wave).clamp(0.15, 0.95);
                }
            }

            equalizer.queue_draw();
            glib::Continue(true)
        });
    }

    // Manejo de clics mejorado
    let player_clone = player.clone();
    let handle_click = move || {
        let player_ref = player_clone.borrow();
        let imp = player_ref.imp();
        let current_idx = *imp.current_index.borrow();

     

        if current_idx == index {
            let state = imp.state.borrow().clone();
            match state {
                PlayerState::Playing => {
                    drop(player_ref);
                    player_clone.borrow().pause();
                }
                PlayerState::Paused | PlayerState::Stopped => {
                    drop(player_ref);
                    player_clone.borrow().play();
                }
            }
        } else {
            *imp.current_index.borrow_mut() = index;
            *imp.is_another_index.borrow_mut() = true;
            drop(player_ref);
            player_clone.borrow().play();
        }
    };

    let handle_click_rc = Rc::new(RefCell::new(handle_click));

    event_box.connect_button_press_event({
        let handle_click = handle_click_rc.clone();
        move |_, event| {
            if event.button() == 1 { // Solo clic izquierdo
                (handle_click.borrow())();
            }
            gtk::Inhibit(false)
        }
    });

    // Separador elegante
    let separator = Separator::new(Orientation::Horizontal);
    separator.style_context().add_class("playlist-separator");
    separator.set_margin_start(72);
    separator.set_margin_end(16);

    main_container.pack_start(&event_box, false, false, 0);
    main_container.pack_start(&separator, false, false, 0);

    main_container
}

// Función mejorada para formatear duración
fn format_duration_enhanced(minutes: f64, seconds: f64) -> String {
    let total_seconds = (minutes * 60.0 + seconds) as u32;
    let mins = total_seconds / 60;
    let secs = total_seconds % 60;
    
    if mins >= 60 {
        let hours = mins / 60;
        let remaining_mins = mins % 60;
        format!("{}:{:02}:{:02}", hours, remaining_mins, secs)
    } else {
        format!("{}:{:02}", mins, secs)
    }
}