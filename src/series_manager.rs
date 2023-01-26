use std::fs::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    pub series_name: String,
    pub series_path: String,
    seasons: Vec<Season>,
    season_watching: i64,
    last_watched: i64,
    time_watched: u32,
}

impl Series {
    fn new() -> Series {
        Series {
            series_name: String::new(),
            series_path: String::new(),
            seasons: Vec::new(),
            season_watching: 0,
            last_watched: 0,
            time_watched: 0,
        }
    }

    fn from_path(path: String) -> Series {
        let series_name = path.split("\\").last().unwrap().to_string();
        let mut seasons: Vec<_> = read_dir(&path).unwrap().map(|r| r.unwrap()).collect();
        seasons.sort_by_key(|dir| dir.file_name().to_str().unwrap().to_string());
        let mut season_num = 1;
        let mut seases: Vec<Season> = Vec::new();
        for season in seasons {
            if metadata(season.path()).unwrap().is_dir(){
                let mut seas = Season::new();
                let mut episodes: Vec<_> = read_dir(season.path()).unwrap().map(|r| r.unwrap()).collect();
                episodes.sort_by_key(|dir| dir.file_name().to_str().unwrap().to_string());
                let mut episode_number = 1;
                for episode in episodes {
                    if metadata(episode.path()).unwrap().is_file(){
                        if episode.file_name().to_str().unwrap().ends_with(".mkv") ||
                        episode.file_name().to_str().unwrap().ends_with(".mp4") ||
                        episode.file_name().to_str().unwrap().ends_with(".avi") {
                            let episode_name = episode.file_name().to_str().unwrap().to_string();
                            let episode_path = episode.path().to_str().unwrap().to_string();
                            let episode = Episode::new(episode_number, episode_name, episode_path);
                            seas.episodes.push(episode);
                            episode_number += 1;
                        }
                    }
                }
                seas.season_number = season_num;
                seas.path = season.path().to_str().unwrap().to_string();
                seases.push(seas);
                season_num += 1;
            }
        }
        return Series {
            series_name,
            series_path: path,
            seasons: seases,
            season_watching: 0,
            last_watched: 0,
            time_watched: 0,
        }
    }


    pub fn save_series(&self){
        let series_json = serde_json::to_string(self).unwrap();
        //Save the json to series folder
        let path = self.series_path.clone() + "\\" + self.series_name.as_str() + ".json";
        std::fs::write(path, series_json).unwrap();
    }


    pub fn verify_series_meta(&self) {
        let path = self.series_path.clone() + "\\" + self.series_name.as_str() + ".json";
        if !std::path::Path::new(&path).exists() {
            self.save_series();
        } else {
            let series_json = std::fs::read_to_string(path).unwrap();
            let series_meta: Series = serde_json::from_str(&series_json).unwrap();
            if series_meta.seasons.len() != self.seasons.len() {
                self.save_series();
            } else {
                for (season, season_meta) in self.seasons.iter().zip(series_meta.seasons.iter()) {
                    if season.episodes.len() != season_meta.episodes.len() {
                        self.save_series();
                        break;
                    }
                }
            }
        }
}


}

#[derive(Debug, Serialize, Deserialize)]
struct Season {
    season_number: u8,
    path: String,
    episodes: Vec<Episode>
}

impl Season {
    fn new() -> Season {
        Season {
            season_number: 0,
            path: String::new(),
            episodes: Vec::new()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Episode {
    episode_number: u8,
    episode_name: String,
    episode_path: String
}

impl Episode {
    fn new(episode_number: u8, episode_name: String, episode_path: String) -> Episode {
        Episode {
            episode_number,
            episode_name,
            episode_path
        }
    }
}
    

pub fn run(path: String){
    let series_name_list = get_series_list(&path);
    for (series_name, series_path) in series_name_list {
        if series_name == "Friends"{
        let series = Series::from_path(series_path);
        let series_json = serde_json::to_string(&series).unwrap();
        println!("{}", series_json);
        }
    }
}

pub fn load_series_meta(series_name: String, series_path: String) -> Series {
    let path = series_path + "\\" + series_name.as_str() + ".json";
    let series_json = std::fs::read_to_string(path.clone()).unwrap();
    let series: Series = serde_json::from_str(&series_json).unwrap();
    let series_load = Series::from_path(path);
    series_load.verify_series_meta();
    series
}

pub fn update_series(series: &mut Series, season: i64, episode:i64, time:u32)-> &mut Series{
    series.season_watching = season;
    series.last_watched = episode;
    series.time_watched = time;
    series.save_series();
    series
}

fn get_series_list(series_root: &str) -> HashMap<String, String> {
    let mut series_list = HashMap::new();
    let serieses = read_dir(series_root).unwrap();
    for series in serieses {
        let series = series.unwrap();
        if metadata(series.path()).unwrap().is_dir(){
            series_list.insert(
                series.file_name().to_str().unwrap().to_string(),
                series.path().to_str().unwrap().to_string()
            );
        }
    }
    series_list
}