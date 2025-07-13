mod types;
mod kernel;
mod loader;
mod gui;
mod widgets;
mod api;
mod utils;
use gtk::prelude::*;
use gtk::Application;
use crate::gui::build_ui;
fn main() {
    let app = Application::new(Some("com.example.music-player"), Default::default());
    app.connect_activate(build_ui);
    app.run();

}
 