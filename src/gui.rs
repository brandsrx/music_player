use crate::types::{SidebarButtons, Song};
use crate::utils::save_data_to_json;
use crate::kernel::PlayerCore;
use crate::widgets::artists::{build_artist_list};
use crate::widgets::setting::create_setting;
use crate::widgets::sidebar::create_sidebar;
use crate::widgets::favorities::create_favorities;
use crate::loader::{load_cover_pixbuf, load_css, scan_music_folder};
use crate::widgets::playlist_widget::create_song_widget;
use crate::widgets::control_bar::create_control_bar;
use gtk::prelude::*;
use gtk::{Application,ApplicationWindow,Box, Stack, StackTransitionType,Orientation};
use std::rc::Rc;
use std::cell::RefCell;
use gdk;
fn build_playlist(player: Rc<RefCell<PlayerCore>>) -> gtk::ScrolledWindow {
    // Crear la ventana desplazable
    let scrolled_window = gtk::ScrolledWindow::new(
        None::<&gtk::Adjustment>,
        None::<&gtk::Adjustment>,
    );
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    load_css("src/css/dark.css");
    // Crear un contenedor vertical para la lista de canciones
    let list_box = gtk::Box::new(gtk::Orientation::Vertical, 5);
    list_box.set_margin_top(10);
    list_box.set_margin_bottom(10);
    list_box.set_margin_start(10);
    list_box.set_margin_end(10);

    // Agregar canciones al contenedor
    let songs = player.borrow().get_playlist().clone();
    for (i, song) in songs.iter().enumerate() {
        let song_widget = create_song_widget(song, i, player.clone());
        list_box.pack_start(&song_widget, false, false, 5);
    }

    // Añadir el contenedor a la ventana desplazable
    scrolled_window.add(&list_box);

    scrolled_window
}

pub fn build_main_layout(player: Rc<RefCell<PlayerCore>>) -> (Box, Stack, SidebarButtons) {
    let main_content = Box::new(Orientation::Horizontal, 0);
    main_content.style_context().add_class("main_container");
    // Sidebar con botones
    let (sidebar, buttons) = create_sidebar();


    // Stack de contenido
    let content_stack = Stack::new();
    content_stack.set_transition_type(StackTransitionType::SlideLeftRight);
    let widget_setting = create_setting();
    let widget_playlist = build_playlist(player.clone());
    let widget_favorites = create_favorities(player);
    let widget_artist = build_artist_list();

    content_stack.add_titled(&widget_playlist, "playlist", "Playlist");
    content_stack.add_titled(&widget_setting, "setting", "Setting");
    content_stack.add_titled(&widget_favorites, "favorities", "Favorities");
    content_stack.add_titled(&widget_artist, "biblioteca", "Biblioteca");

    main_content.pack_start(&sidebar, false, false, 0);
    main_content.pack_start(&content_stack, true, true, 0);

    (main_content, content_stack, buttons)
}
pub fn build_ui(app: &Application) {
    let playlist = match scan_music_folder() {
        Ok(playlist) => playlist,
        Err(e) => {
            eprintln!("Error al escanear carpeta de música: {}", e);
            Vec::new()
        }
    };

    let window = ApplicationWindow::new(app);
    window.set_title("Reproductor musical");
    window.set_default_size(800, 600);
    window.set_resizable(true); // Asegura que se pueda cambiar el tamaño

    let player = Rc::new(RefCell::new(PlayerCore::with_playlist(playlist)));

    // --- Imagen de fondo ---
    let background_image = gtk::Image::new();
    background_image.set_halign(gtk::Align::Fill);
    background_image.set_valign(gtk::Align::Fill);
    background_image.set_hexpand(true);
    background_image.set_vexpand(true);

    // Si hay una canción, cargamos su imagen de fondo
    {
        let song = player.borrow().get_current_song();
        if !song.path_file.is_empty() {
            if let Some(pixbuf) = load_cover_pixbuf(&song.path_file, 1000, 1000) {
                background_image.set_from_pixbuf(Some(&pixbuf));
            }
        }

        // Escalar imagen al tamaño de la ventana
        background_image.connect_size_allocate(move |widget, alloc| {
            if let Some(pixbuf) = widget.pixbuf() {
                let scaled = pixbuf.scale_simple(
                    alloc.width().max(1),
                    alloc.height().max(1),
                    gdk_pixbuf::InterpType::Bilinear,
                );
                widget.set_from_pixbuf(scaled.as_ref());
            }
        });
    }

    // --- Contenido de la app (UI principal) ---
    let ui_container = Box::new(Orientation::Vertical, 0);
    ui_container.set_hexpand(true);
    ui_container.set_vexpand(true);

    let control_bar = create_control_bar(player.clone());
    ui_container.pack_start(&control_bar, false, false, 0);

    let (main_container, content_stack, buttons) = build_main_layout(player.clone());
    ui_container.pack_start(&main_container, true, true, 0);

    // Conectar botones
    {
        let stack = content_stack.clone();
        buttons.btn_playlist.connect_clicked(move |_| {
            stack.set_visible_child_name("playlist");
        });
    }
    {
        let stack = content_stack.clone();
        buttons.btn_settings.connect_clicked(move |_| {
            stack.set_visible_child_name("setting");
        });
    }
    {
        let stack = content_stack.clone();
        buttons.btn_favorites.connect_clicked(move |_| {
            stack.set_visible_child_name("favorities");
        });
    }
    {
        let stack = content_stack.clone();
        buttons.btn_blioteca.connect_clicked(move |_| {
            stack.set_visible_child_name("biblioteca");
        });
    }

 

    let overlay = gtk::Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_vexpand(true);

    let dark_layer = gtk::EventBox::new();
    dark_layer.style_context().add_class("dark-layer");
    dark_layer.set_hexpand(true);
    dark_layer.set_vexpand(true);

    overlay.add(&background_image);       // Fondo
    overlay.add_overlay(&dark_layer);     // Capa oscura semitransparente
    overlay.add_overlay(&ui_container);   // UI encima del fondo

    // CSS provider (hazlo solo una vez al iniciar la app)
    let css = "
    .dark-layer {
        background-color: rgba(0, 0, 0, 0.3);
    }
    ";
    let provider = gtk::CssProvider::new();
    provider.load_from_data(css.as_bytes()).expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error initializing gtk css provider."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // Agregar overlay a la ventana
    window.add(&overlay);
    window.show_all();

    {
    let player: &PlayerCore = &player.borrow();
    let background_image = background_image.clone(); // ✅ Acceso al fondo

    player.connect_local("info-changed", false, move |values| {
        let song = values[1].get::<Song>().expect("msg");

        // --- Actualizar imagen de fondo ---
        if !song.path_file.is_empty() {
            if let Some(pixbuf) = load_cover_pixbuf(&song.path_file, 1000, 1000) {
                background_image.set_from_pixbuf(Some(&pixbuf));

                // Forzar escalado dinámico usando la asignación actual
                let alloc = background_image.allocation();
                let width = alloc.width().max(1);
                let height = alloc.height().max(1);
                let scaled = pixbuf.scale_simple(width, height, gdk_pixbuf::InterpType::Bilinear);
                background_image.set_from_pixbuf(scaled.as_ref());
            }
        } else {
            background_image.clear(); // Si no hay imagen, quitamos el fondo
        }

        None
    });
}


    // Guardar playlist al cerrar
    window.connect_delete_event({
        let player = player.clone();
        move |_, _| {
            let playlist = player.borrow().get_playlist();
            save_data_to_json(&playlist, "src/cache/playlist.json").ok();
            gtk::Inhibit(false)
        }
    });
    {
        buttons.btn_exit.connect_clicked(move |_| {
            window.close();
        });
    }
}
