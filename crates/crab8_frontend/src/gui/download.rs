use std::sync::{Arc, Mutex};

use egui::{Context, Window};
use ehttp::{fetch, Request};

#[derive(Default)]
pub struct DownloadWindow {
    pub open: bool,
    live_url: String,
}

impl DownloadWindow {
    pub fn render(
        &mut self,
        context: &Context,
        rom: Arc<Mutex<Option<Vec<u8>>>>,
        error: Arc<Mutex<Option<String>>>,
    ) {
        let mut closed = false;

        Window::new("Download")
            .open(&mut self.open)
            .show(context, |ui| {
                ui.horizontal(|ui| {
                    ui.label("URL:");
                    ui.text_edit_singleline(&mut self.live_url);
                    if ui.button("Download").clicked() {
                        closed = true;

                        fetch(Request::get(&self.live_url), move |result| {
                            match result {
                                Err(download_error) => match error.lock() {
                                    Err(l_err) => {
                                        log::error!(
                                            "Failed to acquire lock on download error: {l_err}"
                                        );
                                    }

                                    Ok(mut error) => {
                                        *error = Some(download_error);
                                    }
                                },

                                Ok(response) => match rom.lock() {
                                    Err(l_err) => match error.lock() {
                                        Err(el_err) => log::error!(
                                            "Failed to acquire lock on download or error message: {el_err}, {l_err}"
                                        ),

                                        Ok(mut error) => {
                                            *error = Some(format!(
                                                "Failed to acquire lock on rom: {l_err}"
                                            ));
                                        }
                                    },

                                    Ok(mut rom) => {
                                        *rom = Some(response.bytes);
                                    }
                                },
                            };
                        });
                    }
                });
            });

        if closed {
            self.open = false;
        }
    }
}
