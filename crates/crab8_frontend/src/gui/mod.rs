mod about;
mod download;
mod playback;
mod registers;
pub mod renderer;
mod stack;

use std::sync::{Arc, Mutex};

use crab8::Crab8;
use egui::{menu, Context, TopBottomPanel, Window};

use about::AboutWindow;
use playback::PlaybackWindow;
use registers::RegisterWindow;
use rfd::AsyncFileDialog;
use stack::StackWindow;

use self::download::DownloadWindow;

#[derive(Default)]
pub struct Gui {
    about: AboutWindow,
    pub download: DownloadWindow,
    playback: PlaybackWindow,
    registers: RegisterWindow,
    rom: Arc<Mutex<Option<Vec<u8>>>>,
    error: Arc<Mutex<Option<String>>>,
    stack: StackWindow,
}

impl Gui {
    pub fn new() -> Self {
        Self::default()
    }

    fn render(&mut self, context: &Context, crab8: &mut Crab8) {
        TopBottomPanel::top("menu_bar").show(context, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        wasm_bindgen_futures::spawn_local(Self::open_file(
                            self.rom.clone(),
                            self.error.clone(),
                        ));

                        ui.close_menu();
                    }

                    if ui.button("Download").clicked() {
                        self.download.open = !self.download.open;

                        ui.close_menu();
                    }
                });

                ui.menu_button("Debugger", |ui| {
                    if ui.button("Playback Controls").clicked() {
                        self.playback.open = !self.playback.open;

                        ui.close_menu();
                    }
                    if ui.button("Registers").clicked() {
                        self.registers.open = !self.registers.open;

                        ui.close_menu();
                    }

                    if ui.button("Stack").clicked() {
                        self.stack.open = !self.stack.open;

                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.about.open = true;
                        ui.close_menu();
                    }
                })
            })
        });

        match self.rom.lock() {
            Err(err) => println!("Attempted to unlock ROM but it was locked: {err}"),
            Ok(mut loaded_rom) => {
                if let Some(rom) = loaded_rom.clone() {
                    crab8.load(&rom);
                    *loaded_rom = None;
                }
            }
        };

        self.about.render(context);
        self.download
            .render(context, self.rom.clone(), self.error.clone());
        self.playback.render(context, crab8);
        self.registers.render(context, crab8);
        self.stack.render(context, crab8);

        if let Ok(mut error) = self.error.lock() {
            let mut closed = false;

            if let Some(error_message) = error.as_mut() {
                Window::new("Error").open(&mut true).show(context, |ui| {
                    ui.label(error_message.clone());

                    if ui.button("OK").clicked() {
                        closed = true;
                    }
                });
            }

            if closed {
                *error = None;
            }
        }
    }

    async fn open_file(rom: Arc<Mutex<Option<Vec<u8>>>>, error: Arc<Mutex<Option<String>>>) {
        match AsyncFileDialog::new().pick_file().await {
            Some(file) => {
                let file = file.read().await;
                match rom.lock() {
                    Err(l_err) => match error.lock() {
                        Err(el_err) => println!("Failed to lock error: {l_err} {el_err}"),
                        Ok(mut error) => {
                            *error = Some("Failed to load file".into());
                        }
                    },

                    Ok(mut rom) => {
                        *rom = Some(file);
                    }
                }
            }

            None => match error.lock() {
                Err(l_err) => println!("Failed to lock error: {l_err}"),
                Ok(mut error) => {
                    *error = Some("Failed to load file".into());
                }
            },
        };
    }
}
