use std::{
    env, fs::File, io::{Read, Seek, SeekFrom}, mem,
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
    };

    handle_window_events(&mpv, &mk);

}

fn handle_window_events(mpv: &libmpv::Mpv, mk: &MyKeys){
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    loop{
        let keys = device_state.get_keys();
        if keys != prev_keys && keys.len() > 0 {
            if keys[0] == mk.esc {
                return;
            } else if keys[0] == mk.f {
                mpv.set_property("fullscreen", true).unwrap();
            } else if keys[0] == mk.space {
                mpv.pause().unwrap();
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
