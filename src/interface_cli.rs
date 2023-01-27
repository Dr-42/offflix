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
                series.resume_series();
                save_session(&series);
            } else {
                println!("No last session found");
            }
        },
        2 => {
            let series_name = get_last_session();
            if series_name.is_some() {
                let (ser_name, ser_path) = series_list.get_key_value(series_name.unwrap().as_str()).unwrap();
                let mut series = series_manager::load_series_meta(ser_name, ser_path);
                series.next_episode();
                save_session(&series);
            } else {
                println!("No last session found");
            }
        },
        3 => {
            println!("Select a series to watch");
            for (i, (series_name, _)) in series_list.iter().enumerate() {
                println!("\t{}. {}", i + 1, series_name);
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input: i32 = input.trim().parse().unwrap();
            let (ser_name, ser_path) = series_list.get_index(input as usize - 1).unwrap();
            let mut series = series_manager::load_series_meta(ser_name, ser_path);
            println!("What do you wish to do?\n\t1. Resume last watched\n\t2. Watch next episode\n\t3. Select an episode to watch\n\t4. Exit");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input: i32 = input.trim().parse().unwrap();
            match input {
                1 => {
                    series.resume_series();
                    save_session(&series);
                },
                2 => {
                    series.next_episode();
                    save_session(&series);
                },
                3 => {
                    println!("Select season to watch: ");
                    for (i, season) in series.seasons.iter().enumerate() {
                        println!("\t{}. {}", i + 1, season.season_name);
                    }
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input: u64 = input.trim().parse().unwrap();
                    let season = input - 1;
                    println!("Select episode to watch: ");
                    for (i, episode) in series.seasons[season as usize].episodes.iter().enumerate() {
                        println!("\t{}. {}", i + 1, episode.episode_name);
                    }
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input: u64 = input.trim().parse().unwrap();
                    let episode = input - 1;
                    series.watch_episode(season, episode);
                    save_session(&series);
                },
                4 => {
                    println!("Exiting");
                },
                _ => {
                    println!("Invalid choice...Exiting");
                },
            }
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