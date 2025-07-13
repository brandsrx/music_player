use crate::types::{Song, SongFav};
use std::collections::HashMap;
use std::fs::File;
use std::error::Error;
use std::io::{Read};
use std::io::Write;
use serde_json::{to_string_pretty,from_str};

pub fn save_data_to_json(playlist: &Vec<Song>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = to_string_pretty(playlist)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn save_data_to_fav(playlist: &Vec<SongFav>, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = to_string_pretty(playlist)?;
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_data_from_json(path: &str) -> Result<Vec<Song>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let playlist: Vec<Song> = from_str(&content)?;
    Ok(playlist)
}

pub fn load_data_from_fav(path: &str) -> Result<Vec<SongFav>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let playlist: Vec<SongFav> = from_str(&content)?;
    Ok(playlist)
}

pub fn is_json_valid_and_not_empty(path: &str) -> bool {
    if let Ok(content) = std::fs::read_to_string(path) {
        if content.trim().is_empty() {
            return false;
        }
        serde_json::from_str::<serde_json::Value>(&content)
            .map(|v| !v.is_null() && v != serde_json::json!([]))
            .unwrap_or(false)
    } else {
        false
    }
}

pub fn save_map(map_songs :HashMap<String,Vec<SongFav>>,path:&str){
    let json_str = serde_json::to_string_pretty(&map_songs).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(&json_str.as_bytes()).ok();
}
pub fn load_map(path: &str) -> Option<HashMap<String, Vec<SongFav>>> {
    let json_str = std::fs::read_to_string(path).ok()?;
    let map: HashMap<String, Vec<SongFav>> = serde_json::from_str(&json_str).ok()?;
    Some(map)
}