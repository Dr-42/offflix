use std::env;
fn main() {
    let path = format!(
        "filereader://{}",
        env::args()
            .nth(1)
            .expect("Expected path to local media as argument, found nil.")
    );
    offflix::media_player::run(path);
}