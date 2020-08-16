#![feature(proc_macro_hygiene, decl_macro)]

extern crate m3u8_rs;
#[macro_use]
extern crate rocket;

use std::io::Cursor;
use std::fs;
use std::path::PathBuf;
use m3u8_rs::playlist::{
    MasterPlaylist, 
    MediaPlaylist, 
    MediaSegment,
    VariantStream,
    AlternativeMedia,
};
use rocket::http::{ContentType, Status};
use rocket::response::Response;
use tempfile;

#[get("/stream/media/<file_name>")]
fn play_file<'r>(file_name: String) -> Response<'r> {
    println!("play_file called");
    let path = format!("./assets/{}", file_name);
    let filepath = PathBuf::from(path);

    match fs::read(&filepath) {
        Ok(file) => {
            let response = Response::build()
                .status(Status::Ok)
                .header(ContentType::new("application", "x-mpegURL"))
                .raw_header("Accept-Ranges", "bytes")
                .raw_header("Connection", "keep-alive")
                .sized_body(Cursor::new(file))
                .finalize();
            response
        },
        Err(e) => {
            println!("{}", e);
            let response = Response::build()
                .status(Status::Ok)
                .header(ContentType::Plain)
                .sized_body(Cursor::new("Error"))
                .finalize();
            response
        }
    }
}

#[get("/stream/playlist.m3u8")]
fn stream<'r>() -> Response<'r> {
    println!("stream called");
    let path = "./assets/playlist.m3u8";
    let filepath = PathBuf::from(path);

    match fs::read(&filepath) {
        Ok(file) => {
            let response = Response::build()
                .status(Status::Ok)
                .header(ContentType::new("application", "x-mpegURL"))
                .raw_header("Accept-Ranges", "bytes")
                .raw_header("Connection", "keep-alive")
                .sized_body(Cursor::new(file))
                .finalize();
            response
        },
        Err(e) => {
            println!("{}", e);
            let response = Response::build()
                .status(Status::Ok)
                .header(ContentType::Plain)
                .sized_body(Cursor::new("Error"))
                .finalize();
            response
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
        playlist.target_duration = 60.0;
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
        self.playlist.media_sequence = 1;

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
        variant_stream.bandwidth = "7777777".to_string();

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
