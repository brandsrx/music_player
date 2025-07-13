use gtk::prelude::*;
use gtk::{Box, Orientation, Label, Switch, Button, FileChooserButton, FileChooserAction, Separator};
use crate::loader::load_css;
pub fn create_setting() -> gtk::Box {
    let main_container = Box::new(Orientation::Vertical, 0);
    main_container.set_margin_top(24);
    main_container.set_margin_bottom(24);
    main_container.set_margin_start(24);
    main_container.set_margin_end(24);
    main_container.set_hexpand(true);
    main_container.set_vexpand(true);


    // Sección de Apariencia
    let appearance_section = create_section("Apariencia", vec![
        create_theme_control(),
    ]);

    // Sección de Biblioteca
    let library_section = create_section("Biblioteca", vec![
        create_folder_selector(),
        create_action_buttons(),
    ]);

    // Agregar secciones al contenedor principal
    main_container.add(&create_separator());
    main_container.add(&appearance_section);
    main_container.add(&create_separator());
    main_container.add(&library_section);

    main_container
}

fn create_section(title: &str, controls: Vec<gtk::Box>) -> gtk::Box {
    let section = Box::new(Orientation::Vertical, 12);
    section.style_context().add_class("main_container");

    section.set_margin_top(16);
    section.set_margin_bottom(16);

    // Título de la sección
    let title_label = Label::new(Some(title));
    title_label.set_halign(gtk::Align::Start);
    title_label.style_context().add_class("section-title");
    
    section.add(&title_label);

    // Agregar controles
    for control in controls {
        section.add(&control);
    }

    section
}
fn create_theme_control() -> gtk::Box {
    let container = Box::new(Orientation::Horizontal, 12);
    container.set_margin_start(8);
    
    let label = Label::new(Some("Tema oscuro"));
    label.set_halign(gtk::Align::Start);
    label.set_hexpand(true);
    label.style_context().add_class("control-label");
    
    let switch = Switch::new();
    switch.set_halign(gtk::Align::End);
    switch.style_context().add_class("theme-switch");
    switch.set_active(true);
    // ⚙️ Manejar el cambio de tema cuando se cambia el switch
    switch.connect_state_set(|_, is_active| {
        if is_active {
            load_css("src/css/dark.css");
        } else {
            load_css("src/css/light.css");
        }
        // Devuelve true para indicar que manejamos el evento
        Inhibit(false)
    });

    container.add(&label);
    container.add(&switch);
    
    container
}
fn create_folder_selector() -> gtk::Box {
    let container = Box::new(Orientation::Vertical, 8);
    container.set_margin_start(8);
    container.style_context().add_class("main_container");
    let label = Label::new(Some("Carpeta de música"));
    label.set_halign(gtk::Align::Start);
    label.style_context().add_class("control-label");
    
    let chooser = FileChooserButton::new(
        "Seleccionar carpeta",
        FileChooserAction::SelectFolder,
    );
    chooser.set_hexpand(true);
    chooser.style_context().add_class("folder-chooser");
    chooser.set_current_folder("/home/breand/Música");
    chooser.style_context().add_class("main_container");

    container.add(&label);
    container.add(&chooser);
    
    container
}

fn create_action_buttons() -> gtk::Box {
    let container = Box::new(Orientation::Horizontal, 12);
    container.style_context().add_class("main_container");
    container.set_margin_start(8);
    container.set_margin_top(8);
    
    let refresh_button = Button::with_label("Actualizar biblioteca");
    refresh_button.style_context().add_class("action-button2");
    
    let clear_button = Button::with_label("Limpiar historial");
    clear_button.style_context().add_class("action-button2");
    
    container.add(&refresh_button);
    container.add(&clear_button);
    
    container
}

fn create_separator() -> gtk::Separator {
    let separator = Separator::new(Orientation::Horizontal);
    separator.set_margin_top(8);
    separator.set_margin_bottom(8);
    separator.style_context().add_class("section-separator");
    separator
}

