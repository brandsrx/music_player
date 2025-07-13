use crate::types::SidebarButtons;
use gtk::prelude::*;
use gtk::{
    Align, Box as GtkBox, Button, Image, Label, Orientation, Separator};

pub fn create_sidebar_icon(icon_name: &str, fallback_char: &str) -> GtkBox {
    let container = GtkBox::new(Orientation::Horizontal, 0);

    let image = Image::from_icon_name(Some(icon_name), gtk::IconSize::Menu);
    container.add(&image);

    // Si el ícono no existe, usar fallback (detectamos si está vacío)
    if image.storage_type() == gtk::ImageType::Empty {
        container.remove(&image); // Quitar el ícono vacío
        let label = Label::new(Some(fallback_char));
        label.style_context().add_class("icon-fallback");
        container.add(&label);
    }

    container
}

pub fn create_sidebar() -> (GtkBox, SidebarButtons) {
    let sidebar = GtkBox::new(Orientation::Vertical, 0);
    sidebar.set_valign(Align::Fill);
    sidebar.set_hexpand(false);
    sidebar.set_vexpand(true);
    sidebar.set_size_request(220, -1);

    sidebar.style_context().add_class("sidebar-container");

    let nav_section = create_navigation_section();
    let actions_section = create_actions_section();

    sidebar.add(&nav_section);
    sidebar.add(&create_spacer());
    sidebar.add(&actions_section);

    // Botones
    let btn_playlist = create_sidebar_button("Playlist", "folder-music-symbolic", "nav-button");
    let btn_favorites = create_sidebar_button("Favoritos", "emblem-favorite-symbolic", "nav-button");
    let btn_blioteca = create_sidebar_button("Biblioteca", "folder-symbolic", "nav-button");
    let btn_settings = create_sidebar_button("Configuración", "preferences-system-symbolic", "action-button");
    let btn_exit = create_sidebar_button("Salir", "application-exit-symbolic", "action-button exit-button");

    replace_section_buttons(&nav_section, vec![&btn_playlist, &btn_favorites, &btn_blioteca]);
    replace_section_buttons(&actions_section, vec![&btn_settings, &btn_exit]);

    let buttons = SidebarButtons {
        btn_playlist,
        btn_favorites,
        btn_blioteca,
        btn_settings,
        btn_exit,
    };

    (sidebar, buttons)
}

fn create_navigation_section() -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 2);
    section.set_margin_top(20);
    section.set_margin_start(16);
    section.set_margin_end(16);

    let title = Label::new(Some("Biblioteca"));
    title.set_halign(Align::Start);
    title.style_context().add_class("section-title");
    section.add(&title);

    section
}

fn create_actions_section() -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 2);
    section.set_margin_bottom(20);
    section.set_margin_start(16);
    section.set_margin_end(16);

    let separator = Separator::new(Orientation::Horizontal);
    separator.style_context().add_class("section-separator");
    separator.set_margin_top(12);
    separator.set_margin_bottom(12);
    section.add(&separator);

    section
}

fn create_sidebar_button(text: &str, icon_name: &str, css_class: &str) -> Button {
    let button = Button::new();
    button.set_relief(gtk::ReliefStyle::None);
    button.set_halign(Align::Fill);
    button.set_hexpand(true);

    let content = GtkBox::new(Orientation::Horizontal, 12);
    content.set_margin_top(8);
    content.set_margin_bottom(8);
    content.set_margin_start(16);
    content.set_margin_end(16);

    let icon_box = create_sidebar_icon(icon_name, &get_fallback_icon(text));
    icon_box.set_size_request(16, 16);

    let label = Label::new(Some(text));
    label.set_halign(Align::Start);
    label.style_context().add_class("button-text");

    content.add(&icon_box);
    content.add(&label);
    button.add(&content);

    button.style_context().add_class("sidebar-button");
    for class in css_class.split_whitespace() {
        button.style_context().add_class(class);
    }

    button
}

fn get_fallback_icon(text: &str) -> String {
    match text {
        "Playlist" => "♪".to_string(),
        "Favoritos" => "★".to_string(),
        "Artistas" => "♫".to_string(),
        "Álbumes" => "◉".to_string(),
        "Configuración" => "⚙".to_string(),
        "Salir" => "⏻".to_string(),
        _ => "•".to_string(),
    }
}

fn replace_section_buttons(section: &GtkBox, buttons: Vec<&Button>) {
    for button in buttons {
        section.add(button);
    }
}

fn create_spacer() -> GtkBox {
    let spacer = GtkBox::new(Orientation::Vertical, 0);
    spacer.set_vexpand(true);
    spacer
}
