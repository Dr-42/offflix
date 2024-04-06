use std::{error::Error, path::Path};

use super::{InterfaceData, LoadingData, PopUpData, SeriesImages, SeriesViewData};
use crate::{image_downloader, series_manager};
use eframe::egui::{self, Align2, ColorImage, RichText, Vec2};
use egui_extras::RetainedImage;

pub fn loading_ui(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    loading_data: &mut LoadingData,
    interface_data: &mut InterfaceData,
) {
    ui.style_mut().text_styles = interface_data.style.text_styles.clone();
    let name_label = egui::Label::new("OFFFLIX");
    let load_label = egui::Label::new(format!(
        "Loading... {}/{}",
        (loading_data.total_threads - loading_data.thread_count),
        loading_data.total_threads
    ));
    let progress_bar = egui::ProgressBar::new(
        (loading_data.total_threads - loading_data.thread_count) as f32
            / loading_data.total_threads as f32,
    );
    let spin = egui::Spinner::new();
    ui.put(loading_data.name_rect, name_label);
    ui.put(loading_data.loading_text_rect, load_label);
    ui.put(loading_data.progress_bar_rect, progress_bar);
    ui.put(loading_data.spin_rect, spin);
    if interface_data.frame_count == 1 {
        let series_list = series_manager::get_series_list(&interface_data.root);

        for series in series_list {
            let series_image = SeriesImages {
                name: series.0.clone(),
                path: series.1.clone(),
                block: interface_data
                    .images_path
                    .join("blocks")
                    .join(&format!("{}0", series.0.clone()))
                    .with_extension("jpg"),
                //block: format!("images/{}/{}0.jpg", "blocks", series.0.as_str()),
                banner: interface_data
                    .images_path
                    .join("banners")
                    .join(&format!("{}0", series.0.clone()))
                    .with_extension("jpg"),
                //banner: format!("images/{}/{}0.jpg", "banners", series.0.as_str()),
                block_image: None,
                banner_image: None,
            };

            let name = series_image.name.clone();
            let images_path = interface_data
                .images_path
                .clone()
                .to_string_lossy()
                .to_string();
            loading_data.threads.push(std::thread::spawn(move || {
                verify_image(name.as_str(), ImageType::Banner, &images_path).unwrap();
                verify_image(name.clone().as_str(), ImageType::Block, &images_path).unwrap();
            }));
            interface_data.images.push(series_image);
            loading_data.thread_count += 1;
        }
        loading_data.total_threads = loading_data.thread_count;
    }
    for (i, thread) in &mut loading_data.threads.iter().enumerate() {
        if thread.is_finished() && !loading_data.finished.contains(&i) {
            println!("Thread finished {}", i);
            loading_data.finished.push(i);
            loading_data.thread_count -= 1;
        }
    }
    if loading_data.thread_count == 0 && interface_data.frame_count > 1 {
        for image in &mut interface_data.images {
            let banner_image = RetainedImage::from_image_bytes(
                "banner",
                &std::fs::read(image.banner.clone()).unwrap_or_default(),
            );
            let block_image = RetainedImage::from_image_bytes(
                "block",
                &std::fs::read(image.block.clone()).unwrap_or_default(),
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
        interface_data.loading = false;
        loading_data.finished.clear();
    }
    interface_data.frame_count += 1;
    ctx.request_repaint();
}

pub fn series_view(
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    interface_data: &mut InterfaceData,
    series_view_data: &mut SeriesViewData,
    pop_up_data: &mut PopUpData,
) {
    ui.style_mut().text_styles = interface_data.style.text_styles.clone();

    let last_series_name =
        match super::series_manager::get_last_session(&interface_data.session_path) {
            Some(last_series_name) => last_series_name,
            None => pop_up_data.series_list.keys().next().unwrap().to_string(),
        };

    let mut banner_index: usize = 0;

    for (i, img) in interface_data.images.iter().enumerate() {
        if img.name == last_series_name {
            banner_index = i;
            break;
        }
    }

    let banner_resp = ui.put(
        series_view_data.top_banner_rect,
        egui::Image::new(
            interface_data.images[banner_index]
                .banner_image
                .as_ref()
                .unwrap()
                .texture_id(ctx),
            Vec2::new(800.0, 300.0),
        ),
    );
    let label_text = format!(
        "{} : {}",
        "You were watching", interface_data.images[banner_index].name
    );
    //Fill banner label rect with light gray color
    ui.painter().rect_filled(
        series_view_data.banner_label_rect,
        0.,
        egui::Color32::from_rgb(200, 200, 200),
    );
    let _banner_label = ui.put(
        series_view_data.banner_label_rect,
        egui::Label::new(egui::RichText::new(label_text).color(egui::Color32::BLACK)),
    );
    if banner_resp.hovered() && !pop_up_data.win_open {
        let next_button = egui::Button::new("Next");
        let next_button = ui.put(series_view_data.banner_next_rect, next_button);
        if next_button.clicked() {
            let series_name = series_manager::get_last_session(&interface_data.session_path);
            let (ser_name, ser_path);
            if series_name.is_some() {
                (ser_name, ser_path) = pop_up_data
                    .series_list
                    .get_key_value(series_name.unwrap().as_str())
                    .unwrap();
            } else {
                (ser_name, ser_path) = pop_up_data
                    .series_list
                    .get_key_value(interface_data.images[0].name.as_str())
                    .unwrap();
            }
            let mut series =
                series_manager::load_series_meta(ser_name, ser_path, &interface_data.meta_path);
            let next_left = series.next_episode(&interface_data.meta_path);
            if !next_left {
                pop_up_data.info_string =
                    format!("{} : {}", "You have finished watching", series.series_name);
                pop_up_data.info_win_open = true;
            }
            series_manager::save_session(&series, &interface_data.session_path);
        }

        let resume_button = egui::Button::new("Resume");
        let resume_button = ui.put(series_view_data.banner_resume_rect, resume_button);
        if resume_button.clicked() {
            let series_name = series_manager::get_last_session(&interface_data.session_path);
            let (ser_name, ser_path);
            if series_name.is_some() {
                (ser_name, ser_path) = pop_up_data
                    .series_list
                    .get_key_value(series_name.unwrap().as_str())
                    .unwrap();
            } else {
                (ser_name, ser_path) = pop_up_data
                    .series_list
                    .get_key_value(interface_data.images[0].name.as_str())
                    .unwrap();
            }
            let mut series =
                series_manager::load_series_meta(ser_name, ser_path, &interface_data.meta_path);
            series.resume_series(&interface_data.meta_path);
            series_manager::save_session(&series, &interface_data.session_path);
        }

        let random_button = egui::Button::new("Random");
        let random_button = ui.put(series_view_data.banner_random_rect, random_button);
        if random_button.clicked() {
            let series_name = series_manager::get_last_session(&interface_data.session_path);
            let (ser_name, ser_path);
            if series_name.is_some() {
                (ser_name, ser_path) = pop_up_data
                    .series_list
                    .get_key_value(series_name.unwrap().as_str())
                    .unwrap();
            } else {
                (ser_name, ser_path) = pop_up_data
                    .series_list
                    .get_key_value(interface_data.images[0].name.as_str())
                    .unwrap();
            }
            let mut series =
                series_manager::load_series_meta(ser_name, ser_path, &interface_data.meta_path);
            series.play_random_episode(&interface_data.meta_path);
            series_manager::save_session(&series, &interface_data.session_path);
        }
    }
    //Search bar
    let search_bar = egui::TextEdit::singleline(&mut series_view_data.search_bar_buffer)
        .desired_width(290.)
        .hint_text("Search");
    let _search_bar = ui.put(series_view_data.search_bar_rect, search_bar);

    let filtered_series = interface_data
        .images
        .iter()
        .filter(|series| {
            series
                .name
                .to_ascii_lowercase()
                .contains(&series_view_data.search_bar_buffer.to_ascii_lowercase())
        })
        .collect::<Vec<&SeriesImages>>();

    let filtered_series = if filtered_series.is_empty() {
        interface_data.images.iter().collect::<Vec<&SeriesImages>>()
    } else {
        filtered_series
    };

    ui.allocate_ui_at_rect(series_view_data.scroll_area_rect, |ui| {
        egui::ScrollArea::vertical().show_viewport(ui, |ui, _rect| {
            for i in 0..(filtered_series.len() / 3 + 1) {
                ui.horizontal_centered(|ui| {
                    for j in 0..3 {
                        if i * 3 + j >= filtered_series.len() {
                            break;
                        }
                        ui.add_space(series_view_data.block_padding);
                        let image = &filtered_series[i * 3 + j].block_image.as_ref().unwrap();
                        let block_resp = ui.add(egui::Image::new(
                            image.texture_id(ctx),
                            series_view_data.block_size,
                        ));
                        if block_resp.hovered() && !pop_up_data.win_open {
                            ui.allocate_ui_at_rect(block_resp.rect, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.add_space(50.);
                                    ui.label(
                                        RichText::new(filtered_series[i * 3 + j].name.clone())
                                            .color(egui::Color32::WHITE)
                                            .background_color(
                                                egui::Color32::from_rgba_premultiplied(
                                                    0, 0, 40, 180,
                                                ),
                                            ),
                                    );
                                    let res_but = ui.button("resume");
                                    let nex_but = ui.button("next");
                                    let rand_but = ui.button("random");
                                    let sel_res = ui.button("select episode");

                                    if res_but.clicked() {
                                        let (ser_name, ser_path) = pop_up_data
                                            .series_list
                                            .get_key_value(filtered_series[i * 3 + j].name.as_str())
                                            .unwrap();
                                        let mut series = series_manager::load_series_meta(
                                            ser_name,
                                            ser_path,
                                            &interface_data.meta_path,
                                        );
                                        series.resume_series(&interface_data.meta_path);
                                        series_manager::save_session(
                                            &series,
                                            &interface_data.session_path,
                                        );
                                    }

                                    if nex_but.clicked() {
                                        let (ser_name, ser_path) = pop_up_data
                                            .series_list
                                            .get_key_value(filtered_series[i * 3 + j].name.as_str())
                                            .unwrap();
                                        let mut series = series_manager::load_series_meta(
                                            ser_name,
                                            ser_path,
                                            &interface_data.meta_path,
                                        );
                                        let next_left =
                                            series.next_episode(&interface_data.meta_path);
                                        if !next_left {
                                            pop_up_data.info_string =
                                                format!("{} is finished", ser_name);
                                            pop_up_data.info_win_open = true;
                                            println!("{} is finished", ser_name);
                                        }
                                        series_manager::save_session(
                                            &series,
                                            &interface_data.session_path,
                                        );
                                    }

                                    if rand_but.clicked() {
                                        let (ser_name, ser_path) = pop_up_data
                                            .series_list
                                            .get_key_value(filtered_series[i * 3 + j].name.as_str())
                                            .unwrap();
                                        let mut series = series_manager::load_series_meta(
                                            ser_name,
                                            ser_path,
                                            &interface_data.meta_path,
                                        );
                                        series.play_random_episode(&interface_data.meta_path);
                                        series_manager::save_session(
                                            &series,
                                            &interface_data.session_path,
                                        );
                                    }

                                    if sel_res.clicked() {
                                        pop_up_data.win_series =
                                            filtered_series[i * 3 + j].name.clone();
                                        pop_up_data.win_ser_path = pop_up_data
                                            .series_list
                                            .get_key_value(pop_up_data.win_series.as_str())
                                            .unwrap()
                                            .1
                                            .to_string();
                                        let series = series_manager::load_series_meta(
                                            pop_up_data.win_series.as_str(),
                                            pop_up_data.win_ser_path.as_str(),
                                            &interface_data.meta_path,
                                        );

                                        pop_up_data.season_list.clear();
                                        pop_up_data.episode_list.clear();
                                        pop_up_data.season_selected = 0;
                                        pop_up_data.episode_selected = 0;

                                        for season in series.seasons {
                                            pop_up_data
                                                .season_list
                                                .push(season.season_name.clone());
                                            let mut episodes = Vec::new();
                                            for episode in season.episodes {
                                                episodes.push(episode.episode_name.clone());
                                            }
                                            pop_up_data.episode_list.push(episodes);
                                        }
                                        pop_up_data.win_series =
                                            filtered_series[i * 3 + j].name.clone();
                                        pop_up_data.win_open = true;
                                    }
                                });
                            });
                        }
                    }
                    // If less than 3 series are left
                    // resize the scroll area to fit the remaining area
                    let remaining = filtered_series.len() % 3;
                    let padding = series_view_data.block_padding * (remaining as f32 + 1.)
                        + (remaining as f32 + 1.) * series_view_data.block_size.x;
                    ui.add_space(padding);
                });
                ui.add_space(series_view_data.block_padding);
                ui.end_row();
            }

            ui.end_row();
        });
    });
}

pub fn episode_selection_popup(
    ctx: &egui::Context,
    pop_up_data: &mut PopUpData,
    interface_data: &mut InterfaceData,
) {
    egui::Window::new("Select Episode")
        .default_pos(ctx.available_rect().center())
        .fixed_size(Vec2::new(400., 400.))
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            ui.style_mut().text_styles = interface_data.style.text_styles.clone();

            let sea_combo = egui::ComboBox::from_label("Select Season")
                .wrap(true)
                .show_index(
                    ui,
                    &mut pop_up_data.season_selected,
                    pop_up_data.season_list.len(),
                    |i| pop_up_data.season_list[i].to_owned(),
                );

            if sea_combo.changed() {
                pop_up_data.episode_selected = 0;
            }

            let _epi_combo = egui::ComboBox::from_label("Select Episode")
                .wrap(true)
                .show_index(
                    ui,
                    &mut pop_up_data.episode_selected,
                    pop_up_data.episode_list[pop_up_data.season_selected].len(),
                    |i| pop_up_data.episode_list[pop_up_data.season_selected][i].to_owned(),
                );
            ui.vertical_centered(|ui| {
                let pl_but = ui.button("Play");
                let cl_but = ui.button("Close");

                if pl_but.clicked() {
                    let series = pop_up_data
                        .series_list
                        .get_key_value(pop_up_data.win_series.as_str())
                        .unwrap();
                    let mut series = series_manager::load_series_meta(
                        series.0,
                        series.1,
                        &interface_data.meta_path,
                    );
                    series.watch_episode(
                        pop_up_data.season_selected as u64,
                        pop_up_data.episode_selected as u64,
                        &interface_data.meta_path,
                    );
                    series_manager::save_session(&series, &interface_data.session_path);
                    pop_up_data.win_open = false;
                }

                if cl_but.clicked() {
                    pop_up_data.win_open = false;
                }
            });
        });
}

pub fn episode_info_popup(
    ctx: &egui::Context,
    pop_up_data: &mut PopUpData,
    interface_data: &InterfaceData,
) {
    egui::Window::new("Episode Info")
        .default_pos(ctx.available_rect().center())
        .fixed_size(Vec2::new(400., 400.))
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            ui.style_mut().text_styles = interface_data.style.text_styles.clone();
            ui.vertical_centered(|ui| {
                ui.label("Episode Info");
                let cl_but = ui.button("Close");
                if cl_but.clicked() {
                    pop_up_data.info_win_open = false;
                }
            });
        });
}

pub enum ImageType {
    Block,
    Banner,
}

fn verify_image(name: &str, imgtype: ImageType, images_path: &str) -> Result<(), Box<dyn Error>> {
    let path_type = match imgtype {
        ImageType::Banner => "banners",
        ImageType::Block => "blocks",
    }
    .to_string();
    let image_path = format!("{}/{}/{}0", images_path, path_type, name);
    let comp_path = format!("{}.jpg", image_path);
    if !Path::new(&comp_path).exists() {
        println!("{} does not exist", comp_path);
        image_downloader::download(name, &image_path, imgtype);
    }
    Ok(())
}
