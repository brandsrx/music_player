use crate::types::Song;
use crate::kernel::PlayerCore;
use crate::loader::{load_css};
use crate::widgets::playlist_widget::create_song_widget;
use gtk::prelude::*;
use gtk::{ScrolledWindow, Box, Label, Orientation, Align, PolicyType};
use std::rc::Rc;
use std::cell::RefCell;

fn create_no_data_widget() -> ScrolledWindow {
    // Crear el ScrolledWindow principal
    let scrolled_window = gtk::ScrolledWindow::new(
        None::<&gtk::Adjustment>,
        None::<&gtk::Adjustment>,
        );
    scrolled_window.set_policy(PolicyType::Automatic, PolicyType::Automatic);
    scrolled_window.set_hexpand(true);
    scrolled_window.set_vexpand(true);

    // Contenedor principal con padding
    let main_container = Box::new(Orientation::Vertical, 0);
    main_container.set_halign(Align::Fill);
    main_container.set_valign(Align::Fill);
    main_container.set_margin_top(40);
    main_container.set_margin_bottom(40);
    main_container.set_margin_start(40);
    main_container.set_margin_end(40);

    // Contenedor central para centrar el contenido
    let center_box = Box::new(Orientation::Vertical, 24);
    center_box.set_halign(Align::Center);
    center_box.set_valign(Align::Center);
    center_box.set_spacing(24);

    // Crear un ícono usando un símbolo Unicode
    let icon_label = Label::new(Some("🎵"));
    icon_label.set_halign(Align::Center);
    
    // Aplicar CSS para hacer el ícono más grande y con color suave
    let icon_provider = gtk::CssProvider::new();
    icon_provider.load_from_data(b"
        .empty-icon {
            font-size: 48px;
            color: #6a6a6a;
            opacity: 0.7;
        }
    ").unwrap();
    
    let icon_context = icon_label.style_context();
    icon_context.add_provider(&icon_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    icon_context.add_class("empty-icon");

    // Título principal
    let title_label = Label::new(Some("No hay datos agregados"));
    title_label.set_halign(Align::Center);
    
    // CSS para el título estilo Spotify
    let title_provider = gtk::CssProvider::new();
    title_provider.load_from_data(b"
        .empty-title {
            font-size: 24px;
            font-weight: bold;
            color: #ffffff;
            margin-bottom: 8px;
        }
    ").unwrap();
    
    let title_context = title_label.style_context();
    title_context.add_provider(&title_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    title_context.add_class("empty-title");

    // Subtítulo descriptivo
    let subtitle_label = Label::new(Some("Parece que aún no has agregado ningún elemento.\nComienza explorando y agregando tu contenido favorito."));
    subtitle_label.set_halign(Align::Center);
    subtitle_label.set_justify(gtk::Justification::Center);
    subtitle_label.set_line_wrap(true);
    subtitle_label.set_max_width_chars(50);
    
    // CSS para el subtítulo
  let subtitle_provider = gtk::CssProvider::new();
    if let Err(e) = subtitle_provider.load_from_data(b"
        .empty-subtitle {
            font-size: 14px;
            color: #b3b3b3;
        }
    ") {
        eprintln!("Error loading subtitle CSS: {}", e);
    }
    
    let subtitle_context = subtitle_label.style_context();
    subtitle_context.add_provider(&subtitle_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    subtitle_context.add_class("empty-subtitle");

    let general_provider = gtk::CssProvider::new();
    general_provider.load_from_data(b"
        .spotify-dark {
            background-color: #121212;
            color: #ffffff;
        }
        
        .empty-container {
            background-color: #121212;
            border-radius: 8px;
            padding: 40px;
        }
    ").unwrap();

    let main_context = main_container.style_context();
    main_context.add_provider(&general_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    main_context.add_class("spotify-dark");

    let center_context = center_box.style_context();
    center_context.add_provider(&general_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    center_context.add_class("empty-container");

    center_box.pack_start(&icon_label, false, false, 0);
    center_box.pack_start(&title_label, false, false, 0);
    center_box.pack_start(&subtitle_label, false, false, 0);

    main_container.set_center_widget(Some(&center_box));

    scrolled_window.add(&main_container);

    scrolled_window
}
pub fn create_favorities(player: Rc<RefCell<PlayerCore>>) -> gtk::ScrolledWindow {
    let scrolled_window = gtk::ScrolledWindow::new(
        None::<&gtk::Adjustment>,
        None::<&gtk::Adjustment>,
    );
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    load_css("src/css/dark.css");

    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    scrolled_window.add(&container);

    let player_clone = player.clone();
    let container_clone = container.clone();

    let refresh_view = move || {
        for child in container_clone.children() {
            container_clone.remove(&child);
        }

        let songs = player_clone.borrow().get_playlist_fav();

        if songs.is_empty() {
            let no_data = create_no_data_widget();
            container_clone.pack_start(&no_data, true, true, 0);
        } else {
            let list_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
            list_box.set_margin_top(10);
            list_box.set_margin_bottom(10);
            list_box.set_margin_start(10);
            list_box.set_margin_end(10);

            for song in songs.iter() {
                let song_fav: Song = song.clone().into();
                let song_widget = create_song_widget(&song_fav, song.id, player_clone.clone());
                list_box.pack_start(&song_widget, false, false, 5);
            }

            container_clone.pack_start(&list_box, true, true, 0);
        }

        container_clone.show_all();
    };

    refresh_view();

    {
        let refresh_view = refresh_view.clone();
        player.borrow().connect_local("add-remove-favs", false, move |_| {
            refresh_view();
            None
        });
    }

    scrolled_window
}
