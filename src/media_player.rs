use libmpv::{protocol::*, *};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    mem,
};

use crate::key_controls::set_keybindings;

pub fn run(path: String, resume_time: f64) -> (bool, f64) {
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

    match mpv.playlist_load_files(&[(&path, FileState::AppendPlay, None)]) {
        Ok(_) => println!("File loaded successfully"),
        Err(e) => println!("Error loading file: {}", e),
    }

    std::thread::sleep(std::time::Duration::from_millis(1000));
    set_keybindings(&mpv);
    mpv.set_property("keep-open", true).unwrap();
    match mpv.set_property("time-pos", resume_time) {
        Ok(_) => println!("Resuming at {}", resume_time),
        Err(_e) => {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            match mpv.set_property("time-pos", resume_time) {
                Ok(_) => println!("Resuming at {}", resume_time),
                Err(e) => {
                    println!("Error resuming: {}", e)
                }
            }
        }
    }
    mpv.set_property("osc", true).unwrap();
    let (finished, time) = super::key_controls::handle_window_events(&mpv);
    (finished, time)
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

