use crate::types::{Song, SongFav};
use crate::utils::{is_json_valid_and_not_empty, load_data_from_json, save_data_to_json, save_map};
use gdk_pixbuf::Pixbuf;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;
use walkdir::WalkDir;
use lofty::{file::TaggedFileExt, read_from_path};
use lofty::prelude::ItemKey;
use symphonia::core::errors::Error;
use std::path::Path;
use std::fs::File;
use lofty::probe::Probe;
use gtk::prelude::*;
use gdk_pixbuf::InterpType;
use std::collections::HashMap;


pub fn load_css(path: &str) {
    let provider = gtk::CssProvider::new();
    provider
        .load_from_path(path)
        .expect("No se pudo cargar el CSS");

    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::default().expect("No hay pantalla disponible"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn load_cover_pixbuf(path :&str,width:i32,height:i32) ->Option<Pixbuf> {
    let tagged_file = Probe::open(path).ok()?.read().ok()?;

    for tag in tagged_file.tags() {
        for picture in tag.pictures() {
            let image_data = picture.data();
            let loader = gdk_pixbuf::PixbufLoader::new();
            if loader.write(image_data).is_ok() {
                loader.close().ok()?;
                if let Some(pixbuf) = loader.pixbuf() {
                    return pixbuf.scale_simple(width, height, InterpType::Bilinear);
                }
            }
        }
    }
    None
}

fn file_metadata(path_file : &Path)-> Result<Song, Error>{

    let mut song_metadata :Song = Song {
        title: "".to_string(),
        artist:"".to_string(),
        path_file: "".to_string(),
        duration_sec:0.0 ,
        duration_min:0.0 ,
        duration_total_sec:0.0,
        is_favorite:false,
    };

    match read_from_path(path_file) {
        Ok(tag_file) => {
            if let Some(tag) = tag_file.primary_tag() {
                song_metadata.title = tag.get_string(&ItemKey::TrackTitle)
                                    .unwrap_or_else(|| path_file.file_name().unwrap().to_str().unwrap_or("Unknown"))
                                    .to_string();
                song_metadata.artist = tag.get_string(&ItemKey::TrackArtist)
                                    .unwrap_or("Unknown")
                                    .to_string();
            }
        }
        Err(er) => {
            eprintln!("{}",er);
        }
    }
    // Get duration of music
    let file = File::open(path_file)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    // Probar formato
    let hint = Hint::new();
    let probed = get_probe().format(
        &hint,
        mss,
        &Default::default(),
        &MetadataOptions::default(),
    )?;

    let mut format = probed.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.sample_rate.is_some())
        .unwrap();

    let sample_rate = track.codec_params.sample_rate.unwrap();

    let mut total_samples: u64 = 0;

    loop {
        match format.next_packet() {
            Ok(packet) => {
                total_samples += packet.dur;
            }
            Err(Error::ResetRequired) => {
                println!("Reset requerido");
                break;
            }
            Err(Error::IoError(_)) => {
                break;
            }
            Err(e) => {
                println!("Error leyendo paquetes: {:?}", e);
                break;
            }
        }
    }

    // Calcular duración
    let seconds = total_samples as f64 / sample_rate as f64;
    let minutes = (seconds / 60.0).floor();
    let remaining_seconds = seconds % 60.0;
    song_metadata.duration_sec = remaining_seconds;
    song_metadata.duration_min = minutes;
    song_metadata.duration_total_sec = seconds;
    song_metadata.path_file = path_file.to_string_lossy().to_string();
    

    Ok(song_metadata)

}

pub fn scan_music_folder()-> Result<Vec<Song>,Error>{
    let path_cache_playlist = "src/cache/playlist.json";
    
    if is_json_valid_and_not_empty(path_cache_playlist) {
        match load_data_from_json(path_cache_playlist){
            Ok(playlist) =>{
                return Ok(playlist)
            },
            Err(err)=>{
                eprintln!("error al leer la cache : {}",err);
            }
        }
        
    }
    let mut playlist:Vec<Song> = Vec::new();
    let path_folder = "/**";
    let extensions = ["mp3", "flac", "wav", "ogg", "aac"];
    let mut artist_songs:HashMap<String,Vec<SongFav>> = HashMap::new();

    for (index,entry) in WalkDir::new(path_folder)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .enumerate(){
        let path = entry.path();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()){
            if extensions.iter().any(|&x| x.eq_ignore_ascii_case(ext)){
                match file_metadata(path) {
                    Ok(song) => {
                        let song_ref = &song;
                        let song_clone = song_ref.clone();
                        let artist = song_clone.artist;
                        let sf:SongFav =SongFav {
                            id:index,
                            title: song_clone.title,
                            artist: artist.clone(),
                            path_file: song_clone.path_file,
                            duration_sec: song_clone.duration_sec,
                            duration_min: song_clone.duration_min,
                            duration_total_sec: song_clone.duration_total_sec,
                            is_favorite: song_clone.is_favorite
                        };
                        artist_songs
                            .entry(artist)
                            .or_insert_with(Vec::new)
                            .push(sf);
                        playlist.push(song_ref.clone());

                    },
                    Err(e) => { return Err(e) },
                }
            }
        }        
    }
    save_map(artist_songs,"src/cache/artist_songs.json");
    save_data_to_json(&playlist, path_cache_playlist).ok();
    Ok(playlist)
}