use std::fs::*;
use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use serde_json;
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct Series {
    pub series_name: String,
    pub series_path: String,
    pub seasons: Vec<Season>,
    pub season_watching: u64,
    pub last_watched: u64,
    pub time_watched: f64,
    pub series_image: Option<String>,
}

impl Series {
    fn new(path: String) -> Series {
        let series_name = std::path::Path::new(&path).file_name().unwrap().to_str().unwrap().to_string();
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
                            let episode_path = episode.path().to_str().unwrap().replace("\\", "/").to_string();
                            let episode = Episode::new(episode_number, episode_name, episode_path);
                            seas.episodes.push(episode);
                            episode_number += 1;
                        }
                    }
                }
                seas.season_number = season_num;
                seas.path = season.path().to_str().unwrap().to_string();
                seas.season_name = season.file_name().to_str().unwrap().to_string();
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
            time_watched: 0.,
            series_image: None,
        }
    }


    pub fn save_series(&self){
        let series_json = serde_json::to_string(self).unwrap();
        //Save the json to series folder
        let path = "./meta/".to_string() + self.series_name.as_str() + ".json";
        if !std::path::Path::exists(std::path::Path::new("./meta/")){
            std::fs::create_dir_all("./meta/").unwrap();
        }
        std::fs::write(path, series_json).unwrap();
    }


    pub fn verify_series_meta(&self) -> bool{
        for season in &self.seasons {
            if !std::path::Path::new(&season.path).exists() {
                return false;
            }
            for episode in &season.episodes {
                if !std::path::Path::new(episode.episode_path.as_str()).exists() {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn get_episode_path(&self, season: u64, episode: u64) -> String {
        let season = season as usize;
        let episode = episode as usize;
        let mut episode_path = self.seasons[season].episodes[episode].episode_path.clone();
        episode_path = episode_path.replace("\\", "/");
        return episode_path; 
    }

    pub fn resume_series(&mut self) {
        let episode_path = self.get_episode_path(self.season_watching, self.last_watched);
        let (finished , time) = super::media_player::run(episode_path, self.time_watched);
        self.time_watched = time;
        if finished {
            self.next_episode();
        } else {
            self.save_series();
        }
    }

    pub fn next_episode(&mut self) -> bool {
        let episode_path;
        if self.last_watched + 1 == self.seasons[self.season_watching as usize].episodes.len() as u64 {
            if self.season_watching + 1 == self.seasons.len() as u64 {
                println!("No more episodes");
                return false;
            } else {
                self.season_watching += 1;
                self.last_watched = 0;
                episode_path = self.get_episode_path(self.season_watching, self.last_watched);
            }
        } else {
            self.last_watched += 1;
            episode_path = self.get_episode_path(self.season_watching, self.last_watched);
        }
        let (finished , time) = super::media_player::run(episode_path, 0.);
        self.time_watched = time;
        if finished {
            return self.next_episode();
        } else {
            self.save_series();
            return true; 
        }
    }

    pub fn watch_episode(&mut self, season: u64, episode: u64) {
        if season > self.seasons.len() as u64 {
            println!("Season {} does not exist", season);
            return;
        }
        if episode > self.seasons[season as usize].episodes.len() as u64 {
            println!("Episode {} does not exist", episode);
            return;
        }
        self.season_watching = season;
        self.last_watched = episode;
        let episode_path = self.get_episode_path(season, episode);
        let (finished , time) = super::media_player::run(episode_path, 0.);
        self.time_watched = time;
        if finished {
            self.next_episode();
        } else {
            self.save_series();
        }
    }

    pub fn play_random_episode(&mut self) {
        let season = rand::thread_rng().gen_range(0 .. self.seasons.len());
        let episode = rand::thread_rng().gen_range(0 .. self.seasons[season].episodes.len());
        self.season_watching = season as u64;
        self.last_watched = episode as u64;
        let episode_path = self.get_episode_path(season as u64, episode as u64);
        let (finished , time) = super::media_player::run(episode_path, 0.);
        self.time_watched = time;
        if finished {
            self.next_episode();
        } else {
            self.save_series();
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Season {
    pub season_number: u64,
    pub path: String,
    pub episodes: Vec<Episode>,
    pub season_name: String,
}

impl Season {
    fn new() -> Season {
        Season {
            season_number: 0,
            path: String::new(),
            episodes: Vec::new(),
            season_name: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub episode_number: u64,
    pub episode_name: String,
    pub episode_path: String
}

impl Episode {
    fn new(episode_number: u64, episode_name: String, episode_path: String) -> Episode {
        Episode {
            episode_number,
            episode_name,
            episode_path
        }
    }
}
    
pub fn load_series_meta(series_name: &str, series_path: &str) -> Series {
    let path = "meta/".to_string() + series_name + ".json";
    if std::path::Path::new(&path).exists() {
        println!("Loading series meta from: {}", path);
        let series_json = match std::fs::read_to_string(path.clone()){
            Ok(series_json) => series_json,
            Err(_) => {
                println!("Error reading json");
                let series = Series::new(series_path.to_owned());
                series.save_series();
                return series;
            }
        };
        let series: Series = serde_json::from_str(&series_json).unwrap();
        if series.verify_series_meta(){
            return series;
        } else {
            println!("Series meta mismatch, creating new one...");
            let series = Series::new(series_path.to_owned());
            series.save_series();
            return series;
        }
    } else {
        println!("Series meta not found, creating new one...");
        let series = Series::new(series_path.to_owned());
        series.save_series();
        return series;
    }
}

pub fn update_series(series: &mut Series, season: u64, episode:u64, time:f64)-> &mut Series{
    series.season_watching = season;
    series.last_watched = episode;
    series.time_watched = time;
    series.save_series();
    series
}

pub fn get_series_list(series_root: &str) -> IndexMap<String, String> {
    let mut series_list = IndexMap::new();
    let serieses = read_dir(series_root).unwrap();
    for series in serieses {
        let series = series.unwrap();
        if metadata(series.path()).unwrap().is_dir(){
            series_list.insert(
                series.file_name().to_str().unwrap().to_string(),
                series.path().to_str().unwrap().replace("\\", "/").to_string()
            );
        }
    }
    series_list
}

pub fn save_session(series: &Series) {
    let current_series = series.series_name.as_str();
    std::fs::write("session", current_series).unwrap();
}

pub fn get_last_session() -> Option<String> {
    if std::path::Path::new("session").exists() {
        let session = std::fs::read_to_string("session").unwrap();
        return Some(session);
    } 
    None
}
