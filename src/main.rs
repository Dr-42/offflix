use std::env;
fn main() {
    /*let path = format!(
        "filereader://{}",
        env::args()
            .nth(1)
            .expect("Expected path to local media as argument, found nil.")
    );
    offflix::media_player::run(path);*/
    let ser = offflix::series_manager::Series::from_path("G:\\Series\\Friends".to_owned());
    println!("{:#?}", ser);
}