#![windows_subsystem = "windows"]

use directories::ProjectDirs;
use std::path::Path;

fn main() {
    let config_dir = ProjectDirs::from("com", "Dr42Apps", "offflix")
        .unwrap()
        .config_dir()
        .to_owned();
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).expect("Unable to create config directory");
    }

    let root_path = config_dir.join(Path::new("root.conf"));
    if !root_path.exists() {
        offflix::get_root::run(root_path.clone());
    }
    let root = std::fs::read_to_string(&root_path).expect("Unable to read file");
    let root = Path::new(&root).to_owned();
    offflix::interface_gui::run(root, config_dir);
}
