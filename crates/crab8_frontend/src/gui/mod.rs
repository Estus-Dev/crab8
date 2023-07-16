mod about;
mod download;
mod images;
mod memory;
mod playback;
mod registers;
pub mod renderer;

use crab8::Crab8;
use egui::{menu, Context, TopBottomPanel, Window};
use rfd::AsyncFileDialog;
use std::sync::{Arc, Mutex};

use self::{
    about::AboutWindow, download::DownloadWindow, memory::MemoryWindow, playback::PlaybackWindow,
    registers::RegisterWindow,
};

pub struct Gui {
    about: AboutWindow,
    pub download: DownloadWindow,
    playback: PlaybackWindow,
    registers: RegisterWindow,
    rom: Arc<Mutex<Option<Vec<u8>>>>,
    error: Arc<Mutex<Option<String>>>,
    memory: MemoryWindow,
}

impl Gui {
    pub fn new(context: &Context) -> Self {
        Self {
            about: AboutWindow::new(context),
            download: Default::default(),
            playback: Default::default(),
            registers: Default::default(),
            rom: Default::default(),
            error: Default::default(),
            memory: Default::default(),
        }
    }

    fn render(&mut self, context: &Context, crab8: &mut Crab8) {
        TopBottomPanel::top("menu_bar").show(context, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        self.open_file_wrapper();

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

                    if ui.button("Memory").clicked() {
                        self.memory.open = !self.memory.open;

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
            Err(err) => log::error!("Attempted to unlock ROM but it was locked: {err}"),
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
        self.memory.render(context, crab8);
        self.playback.render(context, crab8);
        self.registers.render(context, crab8);

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
                        Err(el_err) => log::error!("Failed to lock error: {l_err} {el_err}"),
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
                Err(l_err) => log::error!("Failed to lock error: {l_err}"),
                Ok(mut error) => {
                    *error = Some("Failed to load file".into());
                }
            },
        };
    }

    #[cfg(not(platform = "wasm32"))]
    fn open_file_wrapper(&mut self) {
        pollster::block_on(Self::open_file(self.rom.clone(), self.error.clone()));
    }

    #[cfg(platform = "wasm32")]
    fn open_file_wrapper(&mut self) {
        wasm_bindgen_futures::spawn_local(Self::open_file(self.rom.clone(), self.error.clone()));
    }
}
