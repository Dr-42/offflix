use eframe::{egui, run_native};

pub fn run() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(400.0, 160.0));
    run_native("Offlix root selector", native_options, Box::new(|cc| Box::new(MyEguiApp::new(cc))));
}

#[derive(Default)]
struct MyEguiApp {
    root: String,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let root = String::new();
        MyEguiApp {root}
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Series root not found!");
            ui.label("Please select the root folder of your series");
            ui.label("The root folder should contain a folder for each series");
            ui.label("Each series folder should contain a folder for each season");
            ui.label("The software auto downloads album art, so active");
            ui.label("internet connection is required. Art can be manually placed as well");
            ui.text_edit_singleline(&mut self.root);
            let button = ui.button("Select");
            if button.clicked() {
                std::fs::write(std::path::Path::new("./root.conf"), self.root.clone()).expect("Unable to write file");
                frame.close();
            }
        });
   }
}