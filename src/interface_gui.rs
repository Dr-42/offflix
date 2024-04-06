use std::path::PathBuf;

use super::series_manager;

pub mod ui_views;

use eframe::{
    egui::{
        self,
        FontFamily::Proportional,
        TextStyle::{Body, Button},
    },
    epaint::FontId,
    run_native,
};
use egui_extras::image::RetainedImage;
use indexmap::IndexMap;

pub struct SeriesImages {
    pub name: String,
    pub path: String,
    pub banner: PathBuf,
    pub block: PathBuf,
    pub banner_image: Option<RetainedImage>,
    pub block_image: Option<RetainedImage>,
}

pub fn run(root: PathBuf, config_dir: PathBuf, cache_dir: PathBuf) {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(800.0, 600.0)),
        resizable: false,
        ..Default::default()
    };
    run_native(
        "Offflix",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc, root, config_dir, cache_dir))),
    );
}

pub struct InterfaceData {
    root: PathBuf,
    meta_path: PathBuf,
    images_path: PathBuf,
    session_path: PathBuf,
    loading: bool,
    images: Vec<SeriesImages>,
    frame_count: usize,
    style: egui::Style,
}

pub struct LoadingData {
    thread_count: usize,
    threads: Vec<std::thread::JoinHandle<()>>,
    total_threads: usize,
    finished: Vec<usize>,
    name_rect: egui::Rect,
    loading_text_rect: egui::Rect,
    progress_bar_rect: egui::Rect,
    spin_rect: egui::Rect,
}

pub struct SeriesViewData {
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
}

pub struct PopUpData {
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

struct MyEguiApp {
    interface_data: InterfaceData,
    loading_data: LoadingData,
    series_view_data: SeriesViewData,
    pop_up_data: PopUpData,
}

impl MyEguiApp {
    fn new(
        cc: &eframe::CreationContext<'_>,
        root: PathBuf,
        config_dir: PathBuf,
        cache_dir: PathBuf,
    ) -> Self {
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
        let images_path = cache_dir.join("images");
        let session_path = config_dir.join("session.conf");

        let interface_data = InterfaceData {
            root,
            meta_path,
            images_path,
            session_path,
            loading,
            images,
            frame_count,
            style,
        };
        let loading_data = LoadingData {
            thread_count,
            threads,
            total_threads,
            finished,
            name_rect,
            loading_text_rect,
            progress_bar_rect,
            spin_rect,
        };
        let series_view_data = SeriesViewData {
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
        };
        let pop_up_data = PopUpData {
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
        };

        MyEguiApp {
            interface_data,
            loading_data,
            series_view_data,
            pop_up_data,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.interface_data.loading {
                ui_views::loading_ui(ui, ctx, &mut self.loading_data, &mut self.interface_data);
            }

            if !self.interface_data.loading {
                ui_views::series_view(
                    ui,
                    ctx,
                    &mut self.interface_data,
                    &mut self.series_view_data,
                    &mut self.pop_up_data,
                );
            }
        });
        if self.pop_up_data.win_open {
            ui_views::episode_selection_popup(ctx, &mut self.pop_up_data, &mut self.interface_data);
        } else if self.pop_up_data.info_win_open {
            ui_views::episode_info_popup(ctx, &mut self.pop_up_data, &self.interface_data);
        }
    }
}
