#![feature(proc_macro_hygiene, decl_macro)]

extern crate m3u8_rs;
#[macro_use]
extern crate rocket;

use std::fs;
use std::path::PathBuf;
use m3u8_rs::playlist::{
    MasterPlaylist, 
    MediaPlaylist, 
    MediaSegment,
    VariantStream,
    AlternativeMedia,
};
use rocket::response::content::Content;
use rocket::http::ContentType;
use tempfile;

#[get("/playlist.m3u8")]
fn play_file() -> Vec<u8> {
    let path = "./assets/playlist.m3u8";
    let filepath = PathBuf::from(path);

    match fs::read(&filepath) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            Vec::new()
        }
    }
}

#[get("/stream/playlist.m3u8")]
fn stream() -> Content<Vec<u8>> {
    let path = "./assets/master_playlist.m3u8";
    let filepath = PathBuf::from(path);

    match fs::read(&filepath) {
        Ok(file) => Content(ContentType::MPEG, file),
        Err(e) => {
            println!("{}", e);
            Content(ContentType::MPEG, Vec::new())
        }
    }
}

fn main() {
    let mut playlist = Playlist::new();
    playlist.add_media_segment();
    playlist.add_master_playlist();

    rocket::ignite()
        .mount("/", routes![play_file, stream])
        .launch();
}

struct Playlist {
    master_playlist: MasterPlaylist,
    playlist: MediaPlaylist,
}

impl Playlist {
    fn new() -> Self {
        let mut playlist = MediaPlaylist::default();
        playlist.version = 3;
        playlist.target_duration = 6.0;
        playlist.media_sequence = 0;

        let mut master_playlist = MasterPlaylist::default();
        master_playlist.version = 6;

        Self {
            master_playlist: master_playlist,
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

    #[allow(unused_must_use)]
    fn add_master_playlist(&mut self) {
        let mut alt_media = AlternativeMedia::default();
        //alt_media.uri = Some("playlist.m3u8".to_string());
        alt_media.group_id = "1".to_string();

        let mut variant_stream = VariantStream::default();
        variant_stream.is_i_frame = false;
        variant_stream.uri = "playlist.m3u8".to_string();
        variant_stream.alternatives.push(alt_media);
        variant_stream.bandwidth = "777777".to_string();

        self.master_playlist.variants.push(variant_stream);

        let hls_root = PathBuf::from("./assets");

        let mut tmp_file = tempfile::Builder::new()
            .prefix("master_playlist.m3u")
            .suffix(".tmp")
            .tempfile_in(hls_root)
            .unwrap();

        self.master_playlist.write_to(&mut tmp_file);
        fs::rename(&tmp_file.path(), "./assets/master_playlist.m3u8");
    }
}
