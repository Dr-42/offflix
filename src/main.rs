use std::{
    env, fs::File, io::{Read, Seek, SeekFrom}, mem, collections::HashMap,
};

use keyboard_query::{self, DeviceQuery, DeviceState};

struct MyKeys {
    esc:u16,
    f:u16,
    space:u16,
    left:u16,
    right:u16,
    up:u16,
    down:u16,
    a:u16,
    v:u16,
    s:u16,
}

fn main() {
    use libmpv::{protocol::*, *};

    let path = format!(
        "filereader://{}",
        env::args()
            .nth(1)
            .expect("Expected path to local media as argument, found nil.")
    );

    let protocol = unsafe {
        Protocol::new(
            "filereader".into(),
            (),
            open,
            close,
            read,
            Some(seek),
            Some(size),
        )
    };

    let mpv = Mpv::new().unwrap();

    let proto_ctx = mpv.create_protocol_context();
    proto_ctx.register(protocol).unwrap();

    mpv.playlist_load_files(&[(&path, FileState::AppendPlay, None)])
        .unwrap();

    let mk = MyKeys {
        esc : 27,
        f : 70,
        space : 32,
        left : 37,
        right : 39,
        up : 38,
        down : 40,
        a : 65,
        v : 86,
        s : 83,
    };

    let total_tracks = mpv.get_property::<i64>("track-list/count").unwrap();
    println!("Total tracks: {}", total_tracks);

    handle_window_events(&mpv, &mk);

}

fn get_tracks(mpv: &libmpv::Mpv) -> HashMap<String, i64>{
    let mut tracks = HashMap::new();
    let track_count = mpv.get_property::<i64>("track-list/count").unwrap();
    println!("Track count : {}", track_count);
    let mut audio_count = 0;
    let mut sub_count = 0;
    let mut video_count = 0;
    for i in 0..track_count {
        let track_type = mpv.get_property::<String>(("track-list/".to_owned() + i.to_string().as_str() + "/type").as_str()).unwrap();
        if track_type == "audio" {
            audio_count += 1;
        } else if track_type == "sub" {
            sub_count += 1;
        } else if track_type == "video" {
            video_count += 1;
        }
    }
    tracks.insert("audio".to_owned(), audio_count);
    tracks.insert("sub".to_owned(), sub_count);
    tracks.insert("video".to_owned(), video_count);
    tracks
}

fn handle_window_events(mpv: &libmpv::Mpv, mk: &MyKeys){
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    let mut paused = false;
    let mut fullscreen = false;

    std::thread::sleep(std::time::Duration::from_millis(1000));
    let track_details = get_tracks(&mpv);
    let mut audio_track = 1;
    let mut sub_track = 1;
    let mut video_track = 1;
    loop{
        let keys = device_state.get_keys();
        if keys != prev_keys && keys.len() > 0 {
            if keys[0] == mk.esc {
                return;
            } else if keys[0] == mk.f {
                if fullscreen {
                    mpv.set_property("fullscreen", false).unwrap();
                    fullscreen = false;
                } else {
                    mpv.set_property("fullscreen", true).unwrap();
                    fullscreen = true;
                }
            } else if keys[0] == mk.space {
                if paused {
                    mpv.unpause().unwrap();
                    paused = false;
                } else {
                    mpv.pause().unwrap();
                    paused = true;
                }
            } else if keys[0] == mk.left {
                mpv.seek_backward(3.).unwrap();
            } else if keys[0] == mk.right {
                mpv.seek_forward(3.).unwrap();
            } else if keys[0] == mk.up {
                let mut vol = mpv.get_property::<i64>("volume").unwrap();
                vol += 5;
                mpv.set_property("volume", vol).unwrap();
            } else if keys[0] == mk.down {
                let mut vol = mpv.get_property::<i64>("volume").unwrap();
                vol -= 5;
                mpv.set_property("volume", vol).unwrap();
            } else if keys[0] == mk.a {
                if audio_track < track_details["audio"] {
                    audio_track += 1;
                } else {
                    audio_track = 1;
                }
                mpv.set_property("aid", audio_track).unwrap();
            } else if keys[0] == mk.v {
                if video_track < track_details["video"] {
                    video_track += 1;
                } else {
                    video_track = 1;
                }
                mpv.set_property("vid", video_track).unwrap();
            } else if keys[0] == mk.s {
                if sub_track < track_details["sub"] {
                    sub_track += 1;
                } else {
                    sub_track = 1;
                }
                mpv.set_property("sid", sub_track).unwrap();
            } else {
                ()
            }
        }
        prev_keys = keys;
    }
}

fn open(_: &mut (), uri: &str) -> File {
    // Open the file, and strip the `filereader://` part
    let ret = File::open(&uri[13..]).unwrap();

    println!("Opened file[{}], ready for orders o7", &uri[13..]);
    ret
}

fn close(_: Box<File>) {
    println!("Closing file, bye bye~~");
}

fn read(cookie: &mut File, buf: &mut [i8]) -> i64 {
    unsafe {
        let forbidden_magic = mem::transmute::<&mut [i8], &mut [u8]>(buf);

        cookie.read(forbidden_magic).unwrap() as _
    }
}

fn seek(cookie: &mut File, offset: i64) -> i64 {
    println!("Seeking to byte {}", offset);
    cookie.seek(SeekFrom::Start(offset as u64)).unwrap() as _
}

fn size(cookie: &mut File) -> i64 {
    cookie.metadata().unwrap().len() as _
}
