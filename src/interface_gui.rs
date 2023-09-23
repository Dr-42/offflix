use std::path::{Path, PathBuf};

use super::series_manager;
use eframe::{
    egui::{
        self,
        FontFamily::Proportional,
        TextStyle::{Body, Button},
    },
    emath::Align2,
    epaint::{ColorImage, FontId, Vec2},
    run_native,
};
use egui_extras::image::RetainedImage;
use indexmap::IndexMap;

pub struct SeriesImages {
    pub name: String,
    pub path: String,
    pub banner: String,
    pub block: String,
    pub banner_image: Option<RetainedImage>,
    pub block_image: Option<RetainedImage>,
}

pub fn run(root: PathBuf, config_dir: PathBuf) {
    let mut native_options = eframe::NativeOptions::default();

    native_options.initial_window_size = Some(egui::Vec2::new(800.0, 600.0));
    native_options.resizable = false;
    run_native(
        "Offflix",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc, root, config_dir))),
    );
}

struct MyEguiApp {
    root: PathBuf,
    meta_path: PathBuf,
    images_path: PathBuf,
    session_path: PathBuf,
    images: Vec<SeriesImages>,
    loading: bool,
    thread_count: usize,
    frame_count: usize,
    threads: Vec<std::thread::JoinHandle<()>>,
    total_threads: usize,
    finished: Vec<usize>,
    name_rect: egui::Rect,
    loading_text_rect: egui::Rect,
    progress_bar_rect: egui::Rect,
    spin_rect: egui::Rect,
    style: egui::Style,
    top_banner_rect: egui::Rect,
    banner_label_rect: egui::Rect,
    banner_next_rect: egui::Rect,
    banner_resume_rect: egui::Rect,
    banner_random_rect: egui::Rect,
    scroll_area_rect: egui::Rect,
    block_size: egui::Vec2,
    search_bar_rect: egui::Rect,
    search_bar_buffer: String,
    block_padding: f32,
    win_open: bool,
    info_win_open: bool,
    info_string: String,
    win_series: String,
    win_ser_path: String,
    season_selected: usize,
    episode_selected: usize,
    season_list: Vec<String>,
    episode_list: Vec<Vec<String>>,
    series_list: IndexMap<String, String>,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>, root: PathBuf, config_dir: PathBuf) -> Self {
        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (Button, FontId::new(24.0, Proportional)),
            (Body, FontId::new(24.0, Proportional)),
        ]
        .into();

        let loading = true;
        let thread_count = 0;
        let frame_count = 0;
        let threads: Vec<std::thread::JoinHandle<()>> = Vec::new();
        let finished: Vec<usize> = Vec::new();
        let total_threads = 0;

        let name_rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(800.0, 300.0));
        let loading_text_rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 300.), egui::Vec2::new(800.0, 200.0));
        let progress_bar_rect =
            egui::Rect::from_min_size(egui::Pos2::new(100.0, 500.), egui::Vec2::new(600.0, 100.0));
        let spin_rect =
            egui::Rect::from_min_size(egui::Pos2::new(350.0, 400.), egui::Vec2::new(100.0, 100.0));

        let top_banner_rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(800.0, 300.0));
        let banner_label_rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 20.0), egui::Vec2::new(460.0, 30.0));
        let banner_next_rect =
            egui::Rect::from_min_size(egui::Pos2::new(690.0, 250.0), egui::Vec2::new(100.0, 30.0));
        let banner_resume_rect =
            egui::Rect::from_min_size(egui::Pos2::new(580.0, 250.0), egui::Vec2::new(100.0, 30.0));
        let banner_random_rect =
            egui::Rect::from_min_size(egui::Pos2::new(470.0, 250.0), egui::Vec2::new(100.0, 30.0));
        let scroll_area_rect =
            egui::Rect::from_min_size(egui::Pos2::new(0.0, 310.0), egui::Vec2::new(800.0, 290.0));
        let block_size = egui::Vec2::new(250.0, 250.0);

        let search_bar_rect =
            egui::Rect::from_min_size(egui::Pos2::new(500.0, 10.0), egui::Vec2::new(290.0, 20.0));
        let search_bar_buffer = String::new();

        let block_padding = 10.;
        let win_open = false;
        let info_win_open = false;
        let info_string = String::new();
        let season_selected = 0;
        let episode_selected = 0;
        let images: Vec<SeriesImages> = Vec::new(); //get_series_images(root.as_str());

        let season_list = Vec::new();
        let episode_list = Vec::new();
        let win_series = String::new();
        let win_ser_path = String::new();

        let series_list = series_manager::get_series_list(&root);

        let meta_path = config_dir.join("meta");
        let images_path = config_dir.join("images");
        let session_path = config_dir.join("session.conf");

        MyEguiApp {
            root,
            meta_path,
            images_path,
            session_path,
            images,
            loading,
            thread_count,
            frame_count,
            threads,
            total_threads,
            finished,
            name_rect,
            loading_text_rect,
            progress_bar_rect,
            spin_rect,
            style,
            top_banner_rect,
            banner_label_rect,
            banner_next_rect,
            banner_resume_rect,
            banner_random_rect,
            scroll_area_rect,
            block_size,
            search_bar_rect,
            search_bar_buffer,
            block_padding,
            win_open,
            info_win_open,
            info_string,
            win_series,
            win_ser_path,
            season_selected,
            episode_selected,
            season_list,
            episode_list,
            series_list,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.loading {
                ui.style_mut().text_styles = self.style.text_styles.clone();
                let name_label = egui::Label::new("OFFFLIX");
                let load_label = egui::Label::new(format!(
                    "Loading... {}/{}",
                    (self.total_threads - self.thread_count),
                    self.total_threads
                ));
                let progress_bar = egui::ProgressBar::new(
                    (self.total_threads - self.thread_count) as f32 / self.total_threads as f32,
                );
                let spin = egui::Spinner::new();
                ui.put(self.name_rect, name_label);
                ui.put(self.loading_text_rect, load_label);
                ui.put(self.progress_bar_rect, progress_bar);
                ui.put(self.spin_rect, spin);
                if self.frame_count == 1 {
                    let series_list = series_manager::get_series_list(&self.root);

                    for series in series_list {
                        let series_image = SeriesImages {
                            name: series.0.clone(),
                            path: series.1.clone(),
                            block: format!("images/{}/{}0.jpg", "blocks", series.0.as_str()),
                            banner: format!("images/{}/{}0.jpg", "banners", series.0.as_str()),
                            block_image: None,
                            banner_image: None,
                        };

                        let name = series_image.name.clone();
                        self.threads.push(std::thread::spawn(move || {
                            verify_image(name.as_str(), ImageType::Banner).unwrap();
                            verify_image(name.clone().as_str(), ImageType::Block).unwrap();
                        }));
                        self.images.push(series_image);
                        self.thread_count += 1;
                    }
                    self.total_threads = self.thread_count;
                }
                for (i, thread) in &mut self.threads.iter().enumerate() {
                    if thread.is_finished() && !self.finished.contains(&i) {
                        println!("Thread finished {}", i);
                        self.finished.push(i);
                        self.thread_count -= 1;
                    }
                }

                /*for i in &self.finished{
                    let thread = self.threads.remove(*i);
                    thread.join().unwrap();
                }*/
                if self.thread_count == 0 && self.frame_count > 1 {
                    for image in &mut self.images {
                        let banner_image = RetainedImage::from_image_bytes(
                            "banner",
                            &std::fs::read(image.banner.clone()).unwrap(),
                        );
                        let block_image = RetainedImage::from_image_bytes(
                            "block",
                            &std::fs::read(image.block.clone()).unwrap(),
                        );
                        match banner_image {
                            Ok(banner_image) => image.banner_image = Some(banner_image),
                            Err(_e) => {
                                image.banner_image =
                                    Some(RetainedImage::from_color_image("", ColorImage::example()))
                            }
                        }
                        match block_image {
                            Ok(block_image) => image.block_image = Some(block_image),
                            Err(_e) => {
                                image.block_image =
                                    Some(RetainedImage::from_color_image("", ColorImage::example()))
                            }
                        }
                    }
                    self.loading = false;
                    self.finished.clear();
                }
                self.frame_count += 1;
                ctx.request_repaint();
            }

            if !self.loading {
                ui.style_mut().text_styles = self.style.text_styles.clone();

                let last_series_name = match super::series_manager::get_last_session() {
                    Some(last_series_name) => last_series_name,
                    None => self.series_list.keys().next().unwrap().to_string(),
                };

                let mut banner_index: usize = 0;

                for (i, img) in self.images.iter().enumerate() {
                    if img.name == last_series_name {
                        banner_index = i;
                        break;
                    }
                }

                let banner_resp = ui.put(
                    self.top_banner_rect,
                    egui::Image::new(
                        self.images[banner_index]
                            .banner_image
                            .as_ref()
                            .unwrap()
                            .texture_id(ctx),
                        Vec2::new(800.0, 300.0),
                    ),
                );
                let label_text = format!(
                    "{} : {}",
                    "You were watching", self.images[banner_index].name
                );
                //Fill banner label rect with light gray color
                ui.painter().rect_filled(
                    self.banner_label_rect,
                    0.,
                    egui::Color32::from_rgb(200, 200, 200),
                );
                let _banner_label = ui.put(
                    self.banner_label_rect,
                    egui::Label::new(egui::RichText::new(label_text).color(egui::Color32::BLACK)),
                );
                if banner_resp.hovered() && !self.win_open {
                    let next_button = egui::Button::new("Next");
                    let next_button = ui.put(self.banner_next_rect, next_button);
                    if next_button.clicked() {
                        let series_name = series_manager::get_last_session();
                        let (ser_name, ser_path);
                        if series_name.is_some() {
                            (ser_name, ser_path) = self
                                .series_list
                                .get_key_value(series_name.unwrap().as_str())
                                .unwrap();
                        } else {
                            (ser_name, ser_path) = self
                                .series_list
                                .get_key_value(self.images[0].name.as_str())
                                .unwrap();
                        }
                        let mut series = series_manager::load_series_meta(ser_name, ser_path);
                        let next_left = series.next_episode();
                        if !next_left {
                            self.info_string = format!(
                                "{} : {}",
                                "You have finished watching", series.series_name
                            );
                            self.info_win_open = true;
                        }
                        series_manager::save_session(&series);
                    }

                    let resume_button = egui::Button::new("Resume");
                    let resume_button = ui.put(self.banner_resume_rect, resume_button);
                    if resume_button.clicked() {
                        let series_name = series_manager::get_last_session();
                        let (ser_name, ser_path);
                        if series_name.is_some() {
                            (ser_name, ser_path) = self
                                .series_list
                                .get_key_value(series_name.unwrap().as_str())
                                .unwrap();
                        } else {
                            (ser_name, ser_path) = self
                                .series_list
                                .get_key_value(self.images[0].name.as_str())
                                .unwrap();
                        }
                        let mut series = series_manager::load_series_meta(ser_name, ser_path);
                        series.resume_series();
                        series_manager::save_session(&series);
                    }

                    let random_button = egui::Button::new("Random");
                    let random_button = ui.put(self.banner_random_rect, random_button);
                    if random_button.clicked() {
                        let series_name = series_manager::get_last_session();
                        let (ser_name, ser_path);
                        if series_name.is_some() {
                            (ser_name, ser_path) = self
                                .series_list
                                .get_key_value(series_name.unwrap().as_str())
                                .unwrap();
                        } else {
                            (ser_name, ser_path) = self
                                .series_list
                                .get_key_value(self.images[0].name.as_str())
                                .unwrap();
                        }
                        let mut series = series_manager::load_series_meta(ser_name, ser_path);
                        series.play_random_episode();
                        series_manager::save_session(&series);
                    }
                }
                //Search bar
                let search_bar = egui::TextEdit::singleline(&mut self.search_bar_buffer)
                    .desired_width(290.)
                    .hint_text("Search");
                let _search_bar = ui.put(self.search_bar_rect, search_bar);

                let filtered_series = self
                    .images
                    .iter()
                    .filter(|series| {
                        series
                            .name
                            .to_ascii_lowercase()
                            .contains(&self.search_bar_buffer.to_ascii_lowercase())
                    })
                    .collect::<Vec<&SeriesImages>>();

                let filtered_series = if filtered_series.is_empty() {
                    self.images.iter().collect::<Vec<&SeriesImages>>()
                } else {
                    filtered_series
                };

                ui.allocate_ui_at_rect(self.scroll_area_rect, |ui| {
                    egui::ScrollArea::vertical().show_viewport(ui, |ui, _rect| {
                        for i in 0..(filtered_series.len() / 3 + 1) {
                            ui.horizontal_centered(|ui| {
                                for j in 0..3 {
                                    if i * 3 + j >= filtered_series.len() {
                                        break;
                                    }
                                    ui.add_space(self.block_padding);
                                    let image =
                                        &filtered_series[i * 3 + j].block_image.as_ref().unwrap();
                                    let block_resp = ui.add(egui::Image::new(
                                        image.texture_id(ctx),
                                        self.block_size,
                                    ));
                                    if block_resp.hovered() && !self.win_open {
                                        ui.allocate_ui_at_rect(block_resp.rect, |ui| {
                                            ui.vertical_centered(|ui| {
                                                ui.add_space(50.);
                                                let lbl = ui
                                                    .label(filtered_series[i * 3 + j].name.clone());
                                                ui.painter().rect_filled(
                                                    lbl.rect,
                                                    0.,
                                                    egui::Color32::from_rgba_premultiplied(
                                                        2, 20, 20, 10,
                                                    ),
                                                );
                                                let res_but = ui.button("resume");
                                                let nex_but = ui.button("next");
                                                let rand_but = ui.button("random");
                                                let sel_res = ui.button("select episode");

                                                if res_but.clicked() {
                                                    let (ser_name, ser_path) = self
                                                        .series_list
                                                        .get_key_value(
                                                            filtered_series[i * 3 + j]
                                                                .name
                                                                .as_str(),
                                                        )
                                                        .unwrap();
                                                    let mut series =
                                                        series_manager::load_series_meta(
                                                            ser_name, ser_path,
                                                        );
                                                    series.resume_series();
                                                    series_manager::save_session(&series);
                                                }

                                                if nex_but.clicked() {
                                                    let (ser_name, ser_path) = self
                                                        .series_list
                                                        .get_key_value(
                                                            filtered_series[i * 3 + j]
                                                                .name
                                                                .as_str(),
                                                        )
                                                        .unwrap();
                                                    let mut series =
                                                        series_manager::load_series_meta(
                                                            ser_name, ser_path,
                                                        );
                                                    let next_left = series.next_episode();
                                                    if !next_left {
                                                        self.info_string =
                                                            format!("{} is finished", ser_name);
                                                        self.info_win_open = true;
                                                        println!("{} is finished", ser_name);
                                                    }
                                                    series_manager::save_session(&series);
                                                }

                                                if rand_but.clicked() {
                                                    let (ser_name, ser_path) = self
                                                        .series_list
                                                        .get_key_value(
                                                            filtered_series[i * 3 + j]
                                                                .name
                                                                .as_str(),
                                                        )
                                                        .unwrap();
                                                    let mut series =
                                                        series_manager::load_series_meta(
                                                            ser_name, ser_path,
                                                        );
                                                    series.play_random_episode();
                                                    series_manager::save_session(&series);
                                                }

                                                if sel_res.clicked() {
                                                    self.win_series =
                                                        filtered_series[i * 3 + j].name.clone();
                                                    self.win_ser_path = self
                                                        .series_list
                                                        .get_key_value(self.win_series.as_str())
                                                        .unwrap()
                                                        .1
                                                        .to_string();
                                                    let series = series_manager::load_series_meta(
                                                        self.win_series.as_str(),
                                                        self.win_ser_path.as_str(),
                                                    );

                                                    self.season_list.clear();
                                                    self.episode_list.clear();
                                                    self.season_selected = 0;
                                                    self.episode_selected = 0;

                                                    for season in series.seasons {
                                                        self.season_list
                                                            .push(season.season_name.clone());
                                                        let mut episodes = Vec::new();
                                                        for episode in season.episodes {
                                                            episodes
                                                                .push(episode.episode_name.clone());
                                                        }
                                                        self.episode_list.push(episodes);
                                                    }
                                                    self.win_series =
                                                        filtered_series[i * 3 + j].name.clone();
                                                    self.win_open = true;
                                                }
                                            });
                                        });
                                    }
                                }
                                // If less than 3 series are left
                                // resize the scroll area to fit the remaining area
                                let remaining = filtered_series.len() % 3;
                                let padding = self.block_padding * (remaining as f32 + 1.)
                                    + (remaining as f32 + 1.) * self.block_size.x;
                                ui.add_space(padding);
                            });
                            ui.add_space(self.block_padding);
                            ui.end_row();
                        }

                        ui.end_row();
                    });
                });
            }
        });
        if self.win_open {
            egui::Window::new("Select Episode")
                .default_pos(ctx.available_rect().center())
                .fixed_size(Vec2::new(400., 400.))
                .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.style_mut().text_styles = self.style.text_styles.clone();

                    let sea_combo = egui::ComboBox::from_label("Select Season")
                        .wrap(true)
                        .show_index(ui, &mut self.season_selected, self.season_list.len(), |i| {
                            self.season_list[i].to_owned()
                        });

                    if sea_combo.changed() {
                        self.episode_selected = 0;
                    }

                    let _epi_combo = egui::ComboBox::from_label("Select Episode")
                        .wrap(true)
                        .show_index(
                            ui,
                            &mut self.episode_selected,
                            self.episode_list[self.season_selected].len(),
                            |i| self.episode_list[self.season_selected][i].to_owned(),
                        );
                    ui.vertical_centered(|ui| {
                        let pl_but = ui.button("Play");
                        let cl_but = ui.button("Close");

                        if pl_but.clicked() {
                            let series = self
                                .series_list
                                .get_key_value(self.win_series.as_str())
                                .unwrap();
                            let mut series = series_manager::load_series_meta(series.0, series.1);
                            series.watch_episode(
                                self.season_selected as u64,
                                self.episode_selected as u64,
                            );
                            series_manager::save_session(&series);
                            self.win_open = false;
                        }

                        if cl_but.clicked() {
                            self.win_open = false;
                        }
                    });
                });
        } else if self.info_win_open {
            egui::Window::new("Info")
                .fixed_size(Vec2::new(400., 400.))
                .show(ctx, |ui| {
                    ui.style_mut().text_styles = self.style.text_styles.clone();
                    ui.vertical_centered(|ui| {
                        ui.label(self.info_string.clone());
                        let cl_but = ui.button("Close");
                        if cl_but.clicked() {
                            self.info_win_open = false;
                        }
                    });
                });
        }
    }
}

enum ImageType {
    Block,
    Banner,
}

pub fn get_series_images(root: PathBuf) -> Vec<SeriesImages> {
    let mut series_images = Vec::new();
    let series_list = series_manager::get_series_list(&root);

    for series in series_list {
        let series_image = SeriesImages {
            name: series.0.clone(),
            path: series.1.clone(),
            block: format!("images/{}/{}0.jpg", "blocks", series.0.as_str()),
            banner: format!("images/{}/{}0.jpg", "banners", series.0.as_str()),
            block_image: None,
            banner_image: None,
        };
        verify_image(&series_image.name, ImageType::Banner).unwrap();
        verify_image(&series_image.name, ImageType::Block).unwrap();
        series_images.push(series_image);
    }
    series_images
}

fn verify_image(name: &str, imgtype: ImageType) -> Result<(), image_search::Error> {
    use image_search::{
        blocking::{download, search, urls},
        Arguments,
    };
    let path_type = match imgtype {
        ImageType::Banner => "banners",
        ImageType::Block => "blocks",
    };
    let image_path = format!("images/{}/{}0.jpg", path_type, name);
    let image_path = Path::new(&image_path);
    if !image_path.exists() {
        println!("{} does not exist", image_path.display());
        match imgtype {
            ImageType::Banner => {
                let args = Arguments::new(name, 1)
                    .ratio(image_search::Ratio::Wide)
                    .format(image_search::Format::Jpg)
                    .directory(PathBuf::from("images/banners")); // Only affects the download function

                let _image_urls = urls(args.clone())?;
                let _images = search(args.clone())?;
                let _paths = download(args)?;
            }
            ImageType::Block => {
                let args = Arguments::new(name, 1)
                    .ratio(image_search::Ratio::Square)
                    .image_type(image_search::ImageType::Photo)
                    .format(image_search::Format::Jpg)
                    .directory(PathBuf::from("images/blocks")); // Only affects the download function

                let _image_urls = urls(args.clone())?;
                let _images = search(args.clone())?;
                let _paths = download(args)?;
            }
        }
    }
    Ok(())
}
