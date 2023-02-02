fn main() {
    if !std::path::Path::new("./root.conf").exists() {
        offflix::get_root::run();
    }
    let root = std::fs::read_to_string(std::path::Path::new("./root.conf")).expect("Unable to read file");
    offflix::interface_gui::run(root);
}