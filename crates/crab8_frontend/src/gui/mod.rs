mod about;
mod playback;
mod registers;
pub mod renderer;
mod stack;

use crab8::Crab8;
use egui::{menu, Context, TopBottomPanel};

use about::AboutWindow;
use playback::PlaybackWindow;
use registers::RegisterWindow;
use stack::StackWindow;

#[derive(Default)]
pub struct Gui {
    about: AboutWindow,
    playback: PlaybackWindow,
    registers: RegisterWindow,
    stack: StackWindow,
}

impl Gui {
    pub fn new() -> Self {
        Self::default()
    }

    fn render(&mut self, context: &Context, crab8: &mut Crab8) {
        TopBottomPanel::top("menu_bar").show(context, |ui| {
            menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))]
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            crab8
                                .load_file(path)
                                .expect("The user would never try to load an invalid file");
                        }

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

        self.about.render(context);
        self.playback.render(context, crab8);
        self.registers.render(context, crab8);
        self.stack.render(context, crab8);
    }
}
