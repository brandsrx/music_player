use crate::types::{Song,SongFav,PlayerState};
use crate::api::api_linux::get_system_volume_linux;
use gstreamer as gst;
use glib::subclass::prelude::*;
use glib::subclass::signal::Signal;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::time::Instant;
use glib::{ StaticType};
use gst::Element; 
use gst::prelude::*; 
pub struct PlayerCoreImp {
    pub playlist: RefCell<Vec<Song>>,
    pub playlist_fav: RefCell<Vec<SongFav>>,
    pub current_index: RefCell<usize>,
    pub is_another_index: RefCell<bool>,
    pub state: RefCell<PlayerState>,
    pub volume: RefCell<f64>, 
    pub playbin: RefCell<Option<Element>>,  // Aquí guardas el playbin de GStreamer
    pub muted: RefCell<bool>,
    pub start_time: RefCell<Instant>,
    pub progress_update_id: RefCell<Option<glib::SourceId>>,
    pub is_updating: RefCell<bool>,
}
#[glib::object_subclass]
impl ObjectSubclass for PlayerCoreImp {
    const NAME: &'static str = "PlayerCore";
    type Type = super::PlayerCore;
    type ParentType = glib::Object;

    fn new() -> Self {
    gst::init().expect("No se pudo inicializar GStreamer");

        let playbin = gst::ElementFactory::make("playbin")
            .name("player")
            .build()
            .expect("Failed to create playbin");

        let current_volume = get_system_volume_linux()
            .map(|v| v as f64 / 100.0)
            .unwrap_or(0.8);

        // Ajustar volumen inicial en playbin
        playbin.set_property("volume", &current_volume);

        Self {
            playbin: RefCell::new(Some(playbin)),
            playlist: RefCell::new(Vec::new()),
            playlist_fav: RefCell::new(Vec::new()),
            current_index: RefCell::new(0),
            volume: RefCell::new(current_volume),
            is_another_index: RefCell::new(false),
            state: RefCell::new(PlayerState::Stopped),
            muted: RefCell::new(false),
            start_time: RefCell::new(Instant::now()),
            progress_update_id: RefCell::new(None),
            is_updating: RefCell::new(false),
        }
    }

}

impl ObjectImpl for PlayerCoreImp {
    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder(
                    "position-changed",
                    &[f64::static_type().into()],
                    <()>::static_type().into(),
                )
                .run_last()
                .build(),
                Signal::builder(
                    "info-changed",
                    &[Song::static_type().into()],
                    <()>::static_type().into(),
                )
                .run_last()
                .build(),
                Signal::builder(
                    "state-changed",
                    &[i32::static_type().into()],
                    <()>::static_type().into(),
                )
                .run_last()
                .build(),
                Signal::builder(
                    "add-remove-favs",
                    &[bool::static_type().into()],
                    <()>::static_type().into(),
                )
                .run_last()
                .build(),
            ]
        });
        SIGNALS.as_ref()
    }

    
}