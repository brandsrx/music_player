# Rust Music Player GUI

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?logo=rust)](https://www.rust-lang.org/)
[![GTK](https://img.shields.io/badge/GTK-3.0-blue?logo=gtk)](https://www.gtk.org/)
[![License](https://img.shields.io/badge/License-MIT-green)](./LICENSE)

A simple music player built in **Rust**, featuring a graphical interface using **GTK 3**, audio playback with **GStreamer** and **Symphonia**, and metadata parsing via **Lofty**.

![Screenshot of Player](./img/screenshot_playlist.png)

## Features

- Supports audio formats: MP3, FLAC, OGG
- Displays album art (if available)
- Recursively scans folders for audio files using `walkdir`
- Extracts metadata (title, artist, album, duration) with `lofty`
- Includes shuffle playback functionality via `fastrand`
- Uses GTK for a modern graphical interface
- Supports multiple input paths through `gio`

## Dependencies

```toml
[dependencies]
gtk = "0.15"
gdk = "0.15"
gdk-pixbuf = "0.15"
gio = "0.15"
glib = "0.15"
cairo-rs = "0.15"
pango = "0.20.12"

symphonia = { version = "0.5.4", features = ["mp3", "flac", "vorbis"] }
gstreamer = "0.23.7"
lofty = "0.22.4"

walkdir = "2"
rand = "0.8"
fastrand = "2.3.0"

serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"
once_cell = "1.21.3"
image = "0.24"
````

## Installation and Build Instructions

### Requirements

Ensure the following are installed:

* Rust (stable or nightly)
* GTK 3
* GStreamer and its plugins

### On Debian/Ubuntu

Install dependencies:

```bash
sudo apt install libgtk-3-dev libgstreamer1.0-dev \
libgstreamer-plugins-base1.0-dev \
gstreamer1.0-plugins-good gstreamer1.0-plugins-ugly gstreamer1.0-libav
```

### On Manjaro/Arch Linux

Install dependencies:

```bash
sudo pacman -S gtk3 gstreamer gst-plugins-base gst-plugins-good \
gst-plugins-bad gst-plugins-ugly gst-libav
```
### Build & Run
```bash
# Debug build
cargo build

# Run project
cargo run
```

## Project Structure

```plaintext
src/
├── types.rs         # types
├── main.rs         # Application bootstrap
├── kernel/         # Audio engine
├── gui/            # Interface components
├── metadata/       # Tag handling
└── utils/          # Helpers
```

## Screenshots

<p align="center">
  <img src="./img/screenshot_playlist.png" alt="Playlist View" width="400"/>
  <img src="./img/screenshot_biblioteca.png" alt="Library View" width="400"/>
</p>

## License

This project is licensed under the MIT License.
See the [LICENSE](./LICENSE) file for details.

