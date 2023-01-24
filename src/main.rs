use std::{
    env,
    fs::File,
    io::{Read, Seek, SeekFrom},
    mem, thread,
    time::Duration,
};

use matroska::{Matroska, Track};
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
    mpv.set_property("volume", 150).unwrap();

    let proto_ctx = mpv.create_protocol_context();
    proto_ctx.register(protocol).unwrap();

    mpv.playlist_load_files(&[(&path, FileState::AppendPlay, None)])
        .unwrap();

    let subs = mpv.get_property::<String>("sid").unwrap();
    println!("Subs: {:?}", subs);
    

    thread::sleep(Duration::from_secs(10));

    mpv.seek_forward(15.).unwrap();

    thread::sleep(Duration::from_secs(5));
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
