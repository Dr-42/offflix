#![windows_subsystem = "windows"]

use directories::ProjectDirs;
use std::path::Path;

mod get_root;
mod image_downloader;
mod interface_gui;
mod key_controls;
mod media_player;
mod series_manager;

fn main() {
    let config_dir = ProjectDirs::from("com", "Dr42Apps", "offflix")
        .unwrap()
        .config_dir()
        .to_owned();
    let cache_dir = ProjectDirs::from("com", "Dr42Apps", "offflix")
        .unwrap()
        .cache_dir()
        .to_owned();
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).expect("Unable to create config directory");
    }

    if !cache_dir.join("images").exists() {
        std::fs::create_dir_all(cache_dir.join("images"))
            .expect("Unable to create cache directory");
    }

    let root_path = config_dir.join(Path::new("root.conf"));
    if !root_path.exists() {
        get_root::run(root_path.clone(), Box::new(false));
    }
    let root = std::fs::read_to_string(&root_path).expect("Unable to read file");
    let root = Path::new(&root).to_owned();

    if !root.exists() {
        get_root::run(root_path.clone(), Box::new(true));
    }

    interface_gui::run(root, config_dir, cache_dir);
}
