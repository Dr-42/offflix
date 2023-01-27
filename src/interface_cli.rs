use crate::media_player;

use super::series_manager::{self, Series};
pub fn run(root: &str){
    let series_list = series_manager::get_series_list(root);
    println!("What do you wish to do?\n\t1. Resume last watched\n\t2. Watch next episode\n\t3. Select a series to watch\n\t4. Exit");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();
    match input {
        1 => {
            let series_name = get_last_session();
            if series_name.is_some() {
                let (ser_name, ser_path) = series_list.get_key_value(series_name.unwrap().as_str()).unwrap();
                println!("{} {}", ser_name, ser_path);
                let mut series = series_manager::load_series_meta(ser_name, ser_path);
                println!("Resuming series: {}", series.series_name);
                println!("Season: {}, Episode: {}", series.season_watching, series.last_watched);
                let episode_path;
                if series.season_watching == 0 {
                    episode_path = series.get_episode_path(0, 0);
                } else {
                    episode_path = series.get_episode_path(series.season_watching - 1, series.last_watched - 1);
                }
                let time = media_player::run(episode_path, series.time_watched);
                series.time_watched = time;
                save_session(&series);
                series.save_series();
            } else {
                println!("No last session found");
            }
        },
        2 => {
            let series_name = get_last_session();
            if series_name.is_some() {
                let (ser_name, ser_path) = series_list.get_key_value(series_name.unwrap().as_str()).unwrap();
                let mut series = series_manager::load_series_meta(ser_name, ser_path);
                println!("Next episode of series: {}", series.series_name);
                let episode_path;
                if series.season_watching == 0 {
                    episode_path = series.get_episode_path(0, 0);
                } else {
                    episode_path = series.get_episode_path(series.season_watching - 1, series.last_watched);
                }
                let time = media_player::run(episode_path, 0.);
                series.last_watched += 1;
                series.time_watched = time;
                save_session(&series);
                series.save_series();
            } else {
                println!("No last session found");
            }
        },
        3 => {
            println!("Select a series to watch");
            let mut series_lst = Vec::new();
            for (i, (series_name, _)) in series_list.iter().enumerate() {
                series_lst.push(series_name);
                println!("\t{}. {}", i+1, series_name);
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input: i32 = input.trim().parse().unwrap();
            let ser_name = series_lst.get(input as usize - 1).unwrap();
            let ser_path = series_list.get(ser_name.as_str()).unwrap();
            let mut series = series_manager::load_series_meta(ser_name, ser_path);
            println!("Select a season to watch");
            for  season in series.seasons.iter() {
                println!("\t{}. Season: {}", season.season_number, season.season_name);
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input: i32 = input.trim().parse().unwrap();
            let season = series.seasons.get(input as usize - 1).unwrap();
            println!("Select an episode to watch");
            for (i, episode) in season.episodes.iter().enumerate() {
                println!("\t{}. Episode: {}", i+1, episode.episode_name);
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input: i32 = input.trim().parse().unwrap();
            let episode = season.episodes.get(input as usize - 1).unwrap();
            let episode_path = episode.episode_path.clone();
            println!("Playing episode: {}", episode_path);
            let time = media_player::run(episode_path, 0.);
            series.last_watched = episode.episode_number;
            series.season_watching = season.season_number;
            series.time_watched = time;
            save_session(&series);
            series.save_series();
        },
        4 => {
            println!("Exiting");
        },
        _ => {
            println!("Invalid choice...Exiting");
        },
    }
}

fn save_session(series: &Series) {
    let current_series = series.series_name.as_str();
    std::fs::write("session", current_series).unwrap();
}

fn get_last_session() -> Option<String> {
    if std::path::Path::new("session").exists() {
        let session = std::fs::read_to_string("session").unwrap();
        return Some(session);
    } 
    None
}