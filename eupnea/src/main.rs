#![feature(proc_macro_hygiene, decl_macro)]

extern crate m3u8_rs;
#[macro_use]
extern crate rocket;

use std::fs;
use std::path::PathBuf;
use m3u8_rs::playlist::{MediaPlaylist, MediaSegment};
use tempfile;

#[get("/playlist.m3u8")]
fn stream() -> Vec<u8> {
    let path = "./assets/sample.ts";
    let filepath = PathBuf::from(path);

    match fs::read(&filepath) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            Vec::new()
        }
    }
}

fn main() {
    let mut playlist = Playlist::new();
    playlist.add_media_segment();

    rocket::ignite()
        .mount("/", routes![stream])
        .launch();
}

struct Playlist {
    playlist: MediaPlaylist,
}

impl Playlist {
    fn new() -> Self {
        let mut playlist = MediaPlaylist::default();
        playlist.version = 3;
        playlist.target_duration = 6.0;
        playlist.media_sequence = 0;

        Self {
            playlist: playlist
        }
    }

    #[allow(unused_must_use)]
    fn add_media_segment(&mut self) {
        let mut segment = MediaSegment::empty();
        segment.duration = 40.0;
        segment.title = Some("".into());
        segment.uri = "sample.ts".to_string();
    
        self.playlist.segments.push(segment);

        let hls_root = PathBuf::from("./assets");

        let mut tmp_file = tempfile::Builder::new()
            .prefix("playlist.m3u")
            .suffix(".tmp")
            .tempfile_in(hls_root)
            .unwrap();

        self.playlist.write_to(&mut tmp_file);
        fs::rename(&tmp_file.path(), "./assets/playlist.m3u8");
    }
}
