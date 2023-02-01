use keyboard_query::{self, DeviceQuery, DeviceState};
use std::collections::HashMap;

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
    ctrl:u16,
    shift:u16,
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
    println!("Audio tracks : {}", audio_count);
    println!("Subtitle tracks : {}", sub_count);
    println!("Video tracks : {}", video_count);
    tracks.insert("audio".to_owned(), audio_count);
    tracks.insert("sub".to_owned(), sub_count);
    tracks.insert("video".to_owned(), video_count);
    tracks
}

pub fn handle_window_events(mpv: &libmpv::Mpv) -> (bool, f64) {

    #[cfg(target_os = "windows")]
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
        ctrl : 17,
        shift : 16,
    };

    #[cfg(target_os = "linux")]
    let mk = MyKeys{
        esc : 1,
        f : 33,
        space : 57,
        left : 105,
        right : 106,
        up : 103,
        down : 108,
        a : 30,
        v : 47,
        s : 31,
        ctrl : 29,
        shift : 42,
    };

    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    let mut paused = false;
    let mut fullscreen = false;

    std::thread::sleep(std::time::Duration::from_millis(1000));
    let track_details = get_tracks(&mpv);
    let mut audio_track = 1;
    let mut sub_track = 1;
    let mut video_track = 1;
    let mut sub_enabled = false;
    let mut audio_enabled = true;

    loop{
        //if media player is closed, return the time
        let end_result = mpv.get_property("eof-reached").unwrap();
        if end_result {
            return (true, 0.0);
        }
        let keys = device_state.get_keys();
        let focus_result = mpv.get_property("focused");
        let focused : bool;
        match focus_result {
            Ok(focus_result) => {
                focused = focus_result;
            },
            Err(_) => {
                focused = false;
            }
        }
        if focused {
            if keys != prev_keys && keys.len() > 0 {
                if keys[0] == mk.esc {
                    let time: f64 = match mpv.get_property("time-pos") {
                        Ok(time) => time,
                        Err(_) => 0.0,
                    };
                    return (false, time);
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
                    mpv.seek_backward(5.).unwrap();
                } else if keys[0] == mk.right {
                    mpv.seek_forward(5.).unwrap();
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
                    sub_enabled = true;
                } else if keys[0] == mk.shift {
                    if keys.len() == 3 {
                        if keys[1] == mk.left {
                            mpv.seek_backward(1.).unwrap();
                        } else if keys[1] == mk.right {
                            mpv.seek_forward(1.).unwrap();
                        }else if keys[1] == mk.s {
                            if sub_track > 1 {
                                sub_track -= 1;
                            } else {
                                sub_track = track_details["sub"];
                            }
                            mpv.set_property("sid", sub_track).unwrap();
                        } else if keys[1] == mk.a {
                            if audio_track > 1 {
                                audio_track -= 1;
                            } else {
                                audio_track = track_details["audio"];
                            }
                            mpv.set_property("aid", audio_track).unwrap();
                        } else if keys[1] == mk.v {
                            if video_track > 1 {
                                video_track -= 1;
                            } else {
                                video_track = track_details["video"];
                            }
                            mpv.set_property("vid", video_track).unwrap();
                        }
                    }
                } else if keys[0] == mk.ctrl {
                    if keys.len() == 3 {
                        if keys[1] == mk.left {
                            mpv.seek_backward(10.).unwrap();
                        } else if keys[1] == mk.right {
                            mpv.seek_forward(10.).unwrap();
                        } else if keys[1] == mk.s {
                            if sub_enabled {
                                mpv.set_property("sid", "no").unwrap();
                                sub_enabled = false;
                            } else {
                                mpv.set_property("sid", sub_track).unwrap();
                                sub_enabled = true;
                            }
                        } else if keys[1] == mk.a {
                            if audio_enabled {
                                mpv.set_property("aid", "no").unwrap();
                                audio_enabled = false;
                            } else {
                                mpv.set_property("aid", audio_track).unwrap();
                                audio_enabled = true;
                            }
                        }
                    }
                } else {
                    ()
                }
            }
            prev_keys = keys;
        }
    }
}
