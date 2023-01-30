use super::series_manager;
use eframe::{
    egui::{self, TextStyle::{Button, Body}, FontFamily::Proportional},
    run_native, epaint::{Vec2, FontId, ColorImage}, emath::Align2};
use egui_extras::image::RetainedImage;
use std::path::{PathBuf, Path};
//use image_search::{Arguments, blocking::{urls, search, download}};

pub struct Series_images{
    name: String,
    path: String,
    block: String,
    banner: String,
}

pub fn run() {
    let mut native_options = eframe::NativeOptions::default();

    native_options.initial_window_size = Some(egui::Vec2::new(800.0, 600.0));
    native_options.resizable = false;
    eframe::run_native("MyApp", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

struct MyEguiApp {
    image: RetainedImage,
    style: egui::Style,
    top_banner_rect : egui::Rect,
    banner_next_rect : egui::Rect,
    banner_resume_rect : egui::Rect,
    banner_random_rect : egui::Rect,
    scroll_area_rect : egui::Rect,
    block_size : egui::Vec2,
    block_padding : f32,
    win_open : bool,
    selected : usize,
    selectables : Vec<String>,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //Load image from path
        //let image = RetainedImage::from_image_bytes(
        //    "banner",
        //    &std::fs::read("images/banners/Avatar - The Last Airbender.jpg").unwrap(),
        //);

        let image = RetainedImage::from_color_image("debug_name",
    ColorImage::example());

        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [(Button, FontId::new(24.0, Proportional)),
                             (Body, FontId::new(24.0, Proportional ))].into();
        let top_banner_rect = egui::Rect::from_min_size(egui::Pos2::new(0.0, 0.0), egui::Vec2::new(800.0, 300.0));
        let banner_next_rect = egui::Rect::from_min_size(egui::Pos2::new(690.0, 250.0), egui::Vec2::new(100.0, 30.0));
        let banner_resume_rect = egui::Rect::from_min_size(egui::Pos2::new(580.0, 250.0), egui::Vec2::new(100.0, 30.0));
        let banner_random_rect = egui::Rect::from_min_size(egui::Pos2::new(470.0, 250.0), egui::Vec2::new(100.0, 30.0));
        let scroll_area_rect = egui::Rect::from_min_size(egui::Pos2::new(0.0, 300.0), egui::Vec2::new(800.0, 300.0));
        let block_size = egui::Vec2::new(250.0, 250.0);
        let block_padding = 10.;
        let win_open = false;
        let mut selected = 0;
        let mut selectables = vec!["One".to_string(), "Two".to_string(), "Three".to_string()];
        MyEguiApp {
            image,
            style,
            top_banner_rect,
            banner_next_rect,
            banner_resume_rect,
            banner_random_rect,
            scroll_area_rect,
            block_size,
            block_padding,
            win_open,
            selected,
            selectables,
         }
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().text_styles = self.style.text_styles.clone();
            
            let banner_resp = ui.put(self.top_banner_rect, 
            egui::Image::new(self.image.texture_id(ctx),
            Vec2::new(800.0, 300.0)));
            if banner_resp.hovered() && !self.win_open {
                
                let next_button = egui::Button::new("Next");
                let next_button = ui.put(self.banner_next_rect, next_button);

                let resume_button = egui::Button::new("Resume");
                let resume_button = ui.put(self.banner_resume_rect, resume_button);

                let random_button = egui::Button::new("Random");
                let random_button = ui.put(self.banner_random_rect, random_button);

                if next_button.clicked() {
                    println!("Next button clicked");
                }
                if resume_button.clicked() {
                    println!("Resume button clicked");
                }
                if random_button.clicked() {
                    println!("Random button clicked");
                }
            }
            ui.allocate_ui_at_rect(self.scroll_area_rect, |ui|{
                egui::ScrollArea::vertical().show_viewport(ui,|ui, rect| {
                    for i in 0..3 {
                        ui.horizontal_centered(|ui| {
                            for j in 0..3 {
                                ui.add_space(self.block_padding);
                                let block_resp = ui.add(egui::Image::new(self.image.texture_id(ctx), self.block_size));
                                if block_resp.hovered() && !self.win_open{
                                    ui.allocate_ui_at_rect(block_resp.rect, |ui|{
                                        ui.vertical_centered(|ui|{
                                            ui.add_space(50.);
                                            ui.button("resume");
                                            ui.button("next");
                                            ui.button("random");
                                            let sel_res = ui.button("select episode");

                                            if sel_res.clicked(){
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
        egui::Window::new("Select Episode")
        .open(&mut self.win_open)
        .default_pos(ctx.available_rect().center())
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui|{
            ui.style_mut().text_styles = self.style.text_styles.clone();
            egui::ComboBox::from_label( "Select Season").show_index(
                ui,
                &mut self.selected,
                self.selectables.len(),
                |i| self.selectables[i].to_owned()
            );
            egui::ComboBox::from_label( "Select Episode").show_index(
                ui,
                &mut self.selected,
                self.selectables.len(),
                |i| self.selectables[i].to_owned()
            );
            ui.vertical_centered(|ui| {
                ui.button("Play");
                ui.button("Cancel");
            });
        });
    }
}



/*enum Image_type{
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
            block: format!("images/{}/{}", series.0.as_str(), "block.png"),
            banner: format!("images/{}/{}", series.0.as_str(), "banner.png"),


        };
        verify_image(&series_image.name, Image_type::Banner).unwrap();
        verify_image(&series_image.name, Image_type::Block).unwrap();
        series_images.push(series_image);
    }
    series_images
}

fn verify_image(name: &str, imgtype: Image_type) -> Result<(), image_search::Error>{

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