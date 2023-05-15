use std::sync::{Arc, Mutex};

use egui::{Context, Window};
use ehttp::{fetch, Request};

#[derive(Default)]
pub struct DownloadWindow {
    pub open: bool,
    live_url: String,
    pub url: Option<String>,
    pub rom: Arc<Mutex<Option<Vec<u8>>>>,
    download_error: Arc<Mutex<Option<String>>>,
}

impl DownloadWindow {
    pub fn render(&mut self, context: &Context) {
        Window::new("Download")
            .open(&mut self.open)
            .show(context, |ui| {
                ui.horizontal(|ui| {
                    ui.label("URL:");
                    ui.text_edit_singleline(&mut self.live_url);
                    if ui.button("Download").clicked() {
                        self.url = Some(self.live_url.clone());
                        let rom = self.rom.clone();
                        let download_error = self.download_error.clone();

                        fetch(Request::get(&self.live_url), move |result| {
                            match result {
                                Err(err) => match download_error.lock() {
                                    Err(l_err) => {
                                        println!(
                                            "Failed to acquire lock on download error: {l_err}"
                                        );
                                    }

                                    Ok(mut download_error) => {
                                        *download_error = Some(err);
                                    }
                                },

                                Ok(response) => match rom.lock() {
                                    Err(l_err) => {
                                        println!("Failed to acquire lock on rom: {l_err}");
                                    }

                                    Ok(mut rom) => {
                                        *rom = Some(response.bytes);
                                    }
                                },
                            };
                        });
                    }
                });
            });
    }
}
