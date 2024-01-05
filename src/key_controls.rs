pub fn set_keybindings(mpv: &libmpv::Mpv) {
    mpv.command("keybind", &["\"ESC\" \"quit\""]).unwrap();
    mpv.command("keybind", &["\"SPACE\" \"cycle pause\""])
        .unwrap();
    mpv.command("keybind", &["\"LEFT\" \"seek -5\""]).unwrap();
    mpv.command("keybind", &["\"RIGHT\" \"seek 5\""]).unwrap();
    mpv.command("keybind", &["\"UP\" \"add volume 5\""])
        .unwrap();
    mpv.command("keybind", &["\"DOWN\" \"add volume -5\""])
        .unwrap();
    mpv.command("keybind", &["\"a\" \"cycle audio\""]).unwrap();
    mpv.command("keybind", &["\"s\" \"cycle sub\""]).unwrap();

    mpv.command("keybind", &["\"SHIFT+s\" \"cycle sub down\""])
        .unwrap();
    mpv.command("keybind", &["\"SHIFT+a\" \"cycle audio down\""])
        .unwrap();
    mpv.command("keybind", &["\"SHIFT+LEFT\" \"seek -1\""])
        .unwrap();
    mpv.command("keybind", &["\"SHIFT+RIGHT\" \"seek 1\""])
        .unwrap();

    mpv.command("keybind", &["\"CTRL+LEFT\" \"seek -10\""])
        .unwrap();
    mpv.command("keybind", &["\"CTRL+RIGHT\" \"seek 10\""])
        .unwrap();

    mpv.command("keybind", &["\"CTRL+a\" \"cycle aid\""])
        .unwrap();
    mpv.command("keybind", &["\"CTRL+s\" \"cycle sid\""])
        .unwrap();
    mpv.command("keybind", &["\"f\" \"cycle fullscreen\""])
        .unwrap();
}

pub fn handle_window_events(mpv: &libmpv::Mpv) -> (bool, f64) {
    let mut prev_time = 0.0;
    let mut event_context = mpv.create_event_context();
    event_context
        .enable_event(libmpv::events::mpv_event_id::Shutdown)
        .unwrap();

    loop {
        if let Some(Ok(libmpv::events::Event::Shutdown)) = event_context.wait_event(0.0) {
            return (false, prev_time);
        }

        prev_time = mpv.get_property("time-pos").unwrap_or(prev_time);
        //if media player is closed, return the time
        let end_result = mpv.get_property("eof-reached").unwrap_or(false);
        if end_result {
            return (true, 0.0);
        };
    }
}
