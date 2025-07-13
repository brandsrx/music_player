use gtk::prelude::*;
use crate::types::SongFav;
use gtk::{Box, Image, Label, ListBox, ListBoxRow, ScrolledWindow, Separator};
use std::collections::HashMap;
use crate::utils::load_map;


pub fn build_artist_list() -> gtk::Box {
    let music_data = load_map("src/cache/artist_songs.json").unwrap();
    // Contenedor principal con estilo Spotify
    let main_box = Box::new(gtk::Orientation::Vertical, 0);
    main_box.style_context().add_class("spotify-main");
    
    // Header con estilo Spotify
    let header = create_spotify_header();
    main_box.pack_start(&header, false, false, 0);
    
    // Separador
    let separator = Separator::new(gtk::Orientation::Horizontal);
    separator.style_context().add_class("spotify-separator");
    main_box.pack_start(&separator, false, false, 0);
    
    // Área de contenido con scroll
    let content_area = create_content_area(&music_data);
    main_box.pack_start(&content_area, true, true, 0);
    
    // Aplicar estilos CSS de Spotify
    apply_spotify_styles();
    
    main_box
}

fn create_spotify_header() -> gtk::Box {
    let header_box = Box::new(gtk::Orientation::Horizontal, 0);
    header_box.style_context().add_class("spotify-header");
    header_box.set_height_request(64);
    
    // Logo/Título con estilo Spotify
    let title_container = Box::new(gtk::Orientation::Horizontal, 12);
    title_container.set_margin_start(24);
    title_container.set_margin_end(24);
    title_container.set_valign(gtk::Align::Center);
    
    // Icono de música
    let music_icon = Image::from_icon_name(Some("folder-music-symbolic"), gtk::IconSize::Button);
    music_icon.style_context().add_class("spotify-logo");
    title_container.pack_start(&music_icon, false, false, 0);
    
    // Título principal
    let title_label = Label::new(Some("Tu biblioteca"));
    title_label.style_context().add_class("spotify-title");
    title_container.pack_start(&title_label, false, false, 0);
    
    header_box.pack_start(&title_container, false, false, 0);
    header_box
}

// fn create_search_bar() -> gtk::Box {
//     let search_container = Box::new(gtk::Orientation::Horizontal, 0);
//     search_container.style_context().add_class("search-container");
//     search_container.set_margin_start(24);
//     search_container.set_margin_end(24);
//     search_container.set_margin_top(16);
//     search_container.set_margin_bottom(16);
    
//     let search_entry = SearchEntry::new();
//     search_entry.style_context().add_class("spotify-search");
//     search_entry.set_placeholder_text(Some("Buscar en Tu biblioteca"));
//     search_entry.set_width_request(300);
    
//     search_container.pack_start(&search_entry, false, false, 0);
//     search_container
// }

fn create_content_area(music_data: &HashMap<String, Vec<SongFav>>) -> ScrolledWindow {
    let scrolled_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    scrolled_window.style_context().add_class("spotify-scroll");
    
    // Contenedor principal del contenido
    let content_box = Box::new(gtk::Orientation::Vertical, 0);
    
    // Sección "Artistas"
    let artists_section = create_artists_section(music_data);
    content_box.pack_start(&artists_section, false, false, 0);
    
    scrolled_window.add(&content_box);
    scrolled_window
}

fn create_artists_section(music_data: &HashMap<String, Vec<SongFav>>) -> gtk::Box {
    let section_box = Box::new(gtk::Orientation::Vertical, 0);
    section_box.style_context().add_class("content-section");
    
    // Header de sección
    let section_header = Box::new(gtk::Orientation::Horizontal, 0);
    section_header.set_margin_start(24);
    section_header.set_margin_end(24);
    section_header.set_margin_top(8);
    section_header.set_margin_bottom(16);
    
    let section_title = Label::new(Some("Artistas"));
    section_title.style_context().add_class("section-title");
    section_header.pack_start(&section_title, false, false, 0);
    
    section_box.pack_start(&section_header, false, false, 0);
    
    // Lista de artistas
    let artists_list = create_artists_list(music_data);
    section_box.pack_start(&artists_list, false, false, 0);
    
    section_box
}

fn create_artists_list(music_data: &HashMap<String, Vec<SongFav>>) -> ListBox {
    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk::SelectionMode::None);
    list_box.style_context().add_class("spotify-list");
    
    // Ordenar artistas alfabéticamente
    let mut artists: Vec<_> = music_data.keys().collect();
    artists.sort();
    
    for artist in artists {
        if let Some(songs) = music_data.get(artist) {
            let artist_item = create_spotify_artist_item(artist, songs);
            list_box.add(&artist_item);
        }
    }
    
    list_box
}

fn create_spotify_artist_item(artist_name: &str, songs: &[SongFav]) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.style_context().add_class("spotify-row");
    
    let container = Box::new(gtk::Orientation::Horizontal, 16);
    container.set_margin_start(24);
    container.set_margin_end(24);
    container.set_margin_top(8);
    container.set_margin_bottom(8);
    
    // Avatar/Imagen del artista (círculo con iniciales)
    let avatar = create_artist_avatar(artist_name);
    container.pack_start(&avatar, false, false, 0);
    
    // Información del artista
    let info_container = Box::new(gtk::Orientation::Vertical, 2);
    info_container.set_valign(gtk::Align::Center);
    
    // Nombre del artista
    let name_label = Label::new(Some(artist_name));
    name_label.style_context().add_class("spotify-artist-name");
    name_label.set_halign(gtk::Align::Start);
    name_label.set_max_width_chars(30);
    
    // Información adicional
    let info_text = format!("Artista • {} canción{}", 
        songs.len(), 
        if songs.len() == 1 { "" } else { "es" }
    );
    
    let info_label = Label::new(Some(&info_text));
    info_label.style_context().add_class("spotify-artist-info");
    info_label.set_halign(gtk::Align::Start);
    
    info_container.pack_start(&name_label, false, false, 0);
    info_container.pack_start(&info_label, false, false, 0);
    container.pack_start(&info_container, true, true, 0);
    
    
    
    row.add(&container);
    row
}

fn create_artist_avatar(artist_name: &str) -> gtk::Box {
    let avatar_container = Box::new(gtk::Orientation::Horizontal, 0);
    avatar_container.style_context().add_class("artist-avatar");
    avatar_container.set_size_request(48, 48);
    
    // Obtener iniciales del artista
    let initials = get_artist_initials(artist_name);
    let initials_label = Label::new(Some(&initials));
    initials_label.style_context().add_class("avatar-text");
    initials_label.set_valign(gtk::Align::Center);
    initials_label.set_halign(gtk::Align::Center);
    
    avatar_container.pack_start(&initials_label, true, true, 0);
    avatar_container
}

fn get_artist_initials(name: &str) -> String {
    name.split_whitespace()
        .take(2)
        .map(|word| word.chars().next().unwrap_or(' ').to_uppercase().collect::<String>())
        .collect::<Vec<String>>()
        .join("")
}

fn apply_spotify_styles() {
    let css_provider = gtk::CssProvider::new();
    
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("Error inicializando pantalla"),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}