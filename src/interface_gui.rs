use std::path::{PathBuf, Path};

use super::series_manager;
use eframe::{
    egui::{self, TextStyle::{Button, Body}, FontFamily::Proportional, Label},
    run_native, epaint::{Vec2, FontId, ColorImage, Color32}, emath::Align2};
use egui_extras::image::RetainedImage;
use indexmap::IndexMap;

pub struct Series_images {
    pub name : String,
    pub path : String,
    pub banner : String,
    pub block : String,
    pub banner_image : Option<RetainedImage>,
    pub block_image : Option<RetainedImage>,
}

pub fn run() {
    let mut native_options = eframe::NativeOptions::default();

    native_options.initial_window_size = Some(egui::Vec2::new(800.0, 600.0));
    native_options.resizable = false;
    run_native("Offflix", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

struct MyEguiApp{
    images: Vec<Series_images>,
    style: egui::Style,
    top_banner_rect : egui::Rect,
    banner_label_rect : egui::Rect,
    banner_next_rect : egui::Rect,
    banner_resume_rect : egui::Rect,
    banner_random_rect : egui::Rect,
    scroll_area_rect : egui::Rect,
    block_size : egui::Vec2,
    block_padding : f32,
    win_open : bool,
    win_series : String,
    season_selected : usize,
    episode_selected : usize,
    season_list : Vec<String>,
    episode_list : Vec<String>,
    series_list : IndexMap<String, String>,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //Load image from path
        //let image = RetainedImage::from_image_bytes(
        //    "banner",
        //    &std::fs::read("images/banners/Avatar - The Last Airbender.jpg").unwrap(),
        //);



        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [(Button, FontId::new(24.0, Proportional)),
                             (Body, FontId::new(24.0, Proportional ))].into();
        let top_banner_rect = egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(800.0, 300.0));
        let banner_label_rect = egui::Rect::from_min_size(egui::Pos2::new(0.0, 20.0), egui::Vec2::new(460.0, 30.0));
        let banner_next_rect = egui::Rect::from_min_size(egui::Pos2::new(690.0, 250.0), egui::Vec2::new(100.0, 30.0));
        let banner_resume_rect = egui::Rect::from_min_size(egui::Pos2::new(580.0, 250.0), egui::Vec2::new(100.0, 30.0));
        let banner_random_rect = egui::Rect::from_min_size(egui::Pos2::new(470.0, 250.0), egui::Vec2::new(100.0, 30.0));
        let scroll_area_rect = egui::Rect::from_min_size(egui::Pos2::new(0.0, 310.0), egui::Vec2::new(800.0, 290.0));
        let block_size = egui::Vec2::new(250.0, 250.0);
        let block_padding = 10.;
        let win_open = false;
        let mut season_selected = 0;
        let mut episode_selected = 0;
        let mut images = get_series_images("G:\\Series");
        for image in &mut images {
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
                Err(e) => image.banner_image = Some(RetainedImage::from_color_image("", ColorImage::example())),
            }
            match block_image {
                Ok(block_image) => image.block_image = Some(block_image),
                Err(e) => image.block_image = Some(RetainedImage::from_color_image("", ColorImage::example())),
            }
        }

        let mut season_list = Vec::new();
        let mut episode_list = Vec::new();
        let mut win_series = String::new();

        let series_list = series_manager::get_series_list("G:\\Series");

        MyEguiApp {
            images,
            style,
            top_banner_rect,
            banner_label_rect,
            banner_next_rect,
            banner_resume_rect,
            banner_random_rect,
            scroll_area_rect,
            block_size,
            block_padding,
            win_open,
            win_series,
            season_selected,
            episode_selected,
            season_list,
            episode_list,
            series_list,
        }
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().text_styles = self.style.text_styles.clone();

            let last_series_name = super::series_manager::get_last_session().unwrap();
            
            let mut banner_index : usize = 0;

            for (i, img) in self.images.iter().enumerate() {
                if img.name == last_series_name {
                    banner_index = i;
                    break;
                }
            }

            let banner_resp = ui.put(self.top_banner_rect,
            egui::Image::new(self.images[banner_index].banner_image.as_ref().unwrap().texture_id(ctx),
            Vec2::new(800.0, 300.0)));
            let label_text = format!("{} : {}", "You were watching", self.images[banner_index].name);
            //Fill banner label rect with light gray color
            ui.painter().rect_filled(self.banner_label_rect,0., egui::Color32::from_rgb(200, 200, 200));
            let banner_label = ui.put(self.banner_label_rect, egui::Label::new(egui::RichText::new(label_text).color(egui::Color32::BLACK)));
            if banner_resp.hovered() && !self.win_open {
                
                let next_button = egui::Button::new("Next");
                let next_button = ui.put(self.banner_next_rect, next_button);
                if next_button.clicked() {
                    let series_name = series_manager::get_last_session();
                    if series_name.is_some() {
                        let (ser_name, ser_path) = self.series_list.get_key_value(series_name.unwrap().as_str()).unwrap();
                        let mut series = series_manager::load_series_meta(ser_name, ser_path);
                        series.next_episode();
                        series_manager::save_session(&series);
                    } else {
                        println!("No last session found");
                    }
                }

                let resume_button = egui::Button::new("Resume");
                let resume_button = ui.put(self.banner_resume_rect, resume_button);
                if resume_button.clicked() {
                    let series_name = series_manager::get_last_session();
                    if series_name.is_some() {
                        let (ser_name, ser_path) = self.series_list.get_key_value(series_name.unwrap().as_str()).unwrap();
                        let mut series = series_manager::load_series_meta(ser_name, ser_path);
                        series.resume_series();
                        series_manager::save_session(&series);
                    } else {
                        println!("No last session found");
                    }
                }

                let random_button = egui::Button::new("Random");
                let random_button = ui.put(self.banner_random_rect, random_button);
                if random_button.clicked() {
                    let series_name = series_manager::get_last_session();
                    if series_name.is_some() {
                        let (ser_name, ser_path) = self.series_list.get_key_value(series_name.unwrap().as_str()).unwrap();
                        let mut series = series_manager::load_series_meta(ser_name, ser_path);
                        series.play_random_episode();
                        series_manager::save_session(&series);
                    } else {
                        println!("No last session found");
                    }
                }
            }

            ui.allocate_ui_at_rect(self.scroll_area_rect, |ui|{
                egui::ScrollArea::vertical().show_viewport(ui,|ui, rect| {
                    for i in 0..(self.images.len() / 3 + 1) {
                        ui.horizontal_centered(|ui| {
                            for j in 0..3 {
                                if i*3 + j >= self.images.len() {
                                    break;
                                }
                                ui.add_space(self.block_padding);
                                let image = &self.images[i * 3 + j].block_image.as_ref().unwrap();
                                let block_resp = ui.add(egui::Image::new(image.texture_id(ctx), self.block_size));
                                if block_resp.hovered() && !self.win_open{
                                    ui.allocate_ui_at_rect(block_resp.rect, |ui|{
                                        ui.vertical_centered(|ui|{
                                            ui.add_space(50.);
                                            let lbl = ui.label(self.images[i * 3 + j].name.clone());
                                            ui.painter().rect_filled(lbl.rect,0., egui::Color32::from_rgba_premultiplied(2, 20, 20, 10));
                                            let res_but = ui.button("resume");
                                            let nex_but = ui.button("next");
                                            let rand_but = ui.button("random");
                                            let sel_res = ui.button("select episode");

                                            if res_but.clicked(){
                                                let (ser_name, ser_path) = self.series_list.get_key_value(self.images[i * 3 + j].name.as_str()).unwrap();
                                                let mut series = series_manager::load_series_meta(ser_name, ser_path);
                                                series.resume_series();
                                                series_manager::save_session(&series);
                                            }

                                            if nex_but.clicked(){
                                                let (ser_name, ser_path) = self.series_list.get_key_value(self.images[i * 3 + j].name.as_str()).unwrap();
                                                let mut series = series_manager::load_series_meta(ser_name, ser_path);
                                                series.next_episode();
                                                series_manager::save_session(&series);
                                            }

                                            if rand_but.clicked(){
                                                let (ser_name, ser_path) = self.series_list.get_key_value(self.images[i * 3 + j].name.as_str()).unwrap();
                                                let mut series = series_manager::load_series_meta(ser_name, ser_path);
                                                series.play_random_episode();
                                                series_manager::save_session(&series);
                                            }

                                            if sel_res.clicked(){
                                                self.win_series = self.images[i * 3 + j].name.clone();
                                                self.win_open = true;
                                            }
                                        });
                                    });
                                }
                            }
                        });
                        ui.add_space(self.block_padding);
                        ui.end_row();
                    }
                    ui.end_row();
                });
            });
        });
        if self.win_open {
            egui::Window::new("Select Episode")
            .default_pos(ctx.available_rect().center())
            .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui|{
                ui.style_mut().text_styles = self.style.text_styles.clone();

                let season = self.series_list.get_key_value(self.win_series.as_str()).unwrap().1;
                let mut series = series_manager::load_series_meta(self.win_series.as_str(), season);

                for season in &series.seasons{
                    self.season_list.push(season.season_name.clone());
                }
                
                egui::ComboBox::from_label( "Select Season").show_index(
                    ui,
                    &mut self.season_selected,
                    self.season_list.len(),
                    |i| self.season_list[i].to_owned()
                );

                let seas = &series.seasons[self.season_selected];

                for episode in &seas.episodes[..]{
                    self.episode_list.push(episode.episode_name.clone());
                }            
                egui::ComboBox::from_label( "Select Episode").show_index(
                    ui,
                    &mut self.episode_selected,
                    self.episode_list.len(),
                    |i| self.episode_list[i].to_owned()
                );
                ui.vertical_centered(|ui| {
                    let pl_but = ui.button("Play");
                    let cl_but = ui.button("Close");

                    if pl_but.clicked(){
                        series.watch_episode(self.season_selected as u64, self.episode_selected as u64);
                        series_manager::save_session(&series);
                        self.win_open = false;
                    }

                    if cl_but.clicked(){
                        self.win_open = false;
                    }
                });
            });
        }
    }
}



enum Image_type{
    Block,
    Banner,
}

pub fn get_series_images(root: &str)->Vec<Series_images>{
    let mut series_images = Vec::new();
    let series_list = series_manager::get_series_list(root);

    for series in series_list{
        let series_image = Series_images{
            name: series.0.clone(),
            path: series.1.clone(),
            block: format!("images/{}/{}0.jpg", "blocks", series.0.as_str()),
            banner: format!("images/{}/{}0.jpg", "banners", series.0.as_str()),
            block_image: None,
            banner_image: None,
        };
        //verify_image(&series_image.name, Image_type::Banner).unwrap();
        //verify_image(&series_image.name, Image_type::Block).unwrap();
        series_images.push(series_image);
    }
    series_images
}

/*fn verify_image(name: &str, imgtype: Image_type) -> Result<(), image_search::Error>{
    use image_search::{Arguments, Time, blocking::{urls, search, download}};
        let path_type = match imgtype{
        Image_type::Banner => "banners",
        Image_type::Block => "blocks",
    };
    let image_path = format!("images/{}/{}0.jpg",path_type, name);
    let image_path = Path::new(&image_path);
    if !image_path.exists(){
        println!("{} does not exist", image_path.display());
        match imgtype{
            Image_type::Banner => {
            let args = Arguments::new(name, 1)
            .ratio(image_search::Ratio::Wide)
            .format(image_search::Format::Jpg)
            .directory(PathBuf::from("images/banners")); // Only affects the download function
        
            let _image_urls = urls(args.clone())?;
            let _images = search(args.clone())?;
            let _paths = download(args)?;
            },
            Image_type::Block => {
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

}*/