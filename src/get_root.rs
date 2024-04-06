use eframe::{egui, run_native};
use std::path::PathBuf;

pub fn run(root_path: PathBuf, precofigured: Box<bool>) {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(430.0, 170.0)),
        ..Default::default()
    };
    run_native(
        "Offlix root selector",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc, root_path, precofigured))),
    );
}

#[derive(Default)]
struct MyEguiApp {
    root: String,
    root_path: PathBuf,
    precofigured: Box<bool>,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>, root_path: PathBuf, precofigured: Box<bool>) -> Self {
        let root = String::new();
        MyEguiApp {
            root,
            root_path,
            precofigured,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if !*self.precofigured {
                ui.heading("Series root not found!");
                ui.label("Please select the root folder of your series");
                ui.label("The root folder should contain a folder for each series");
                ui.label("Each series folder should contain a folder for each season");
                ui.label("The software auto downloads album art, so active");
                ui.label("internet connection is required. Art can be manually placed as well");
            } else {
                ui.heading("Could not find the series root folder");
                ui.label("Check if the root folder is accessible");
                ui.label("If the root folder has moved, select the new location");
                ui.label("Otherwise, close the window and make sure the root folder is accessible");
                ui.label("The root folder should contain a folder for each series");
                ui.label("Each series folder should contain a folder for each season");
                ui.label("The software auto downloads album art, so active");
                ui.label("internet connection is required. Art can be manually placed as well");
            }
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.root);
                let browse = ui.button("Browse");
                if browse.clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.root = path.to_string_lossy().to_string();
                    }
                }
                let button = ui.button("Select");
                if button.clicked() && !self.root.is_empty() {
                    std::fs::write(&self.root_path, self.root.clone())
                        .expect("Unable to write file");
                    frame.close();
                }
            });
        });
    }
}
