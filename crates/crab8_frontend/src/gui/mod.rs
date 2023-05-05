mod about;
pub mod renderer;

use crab8::Crab8;
use egui::{menu, Context, TopBottomPanel};

use about::AboutWindow;

#[derive(Default)]
pub struct Gui {
    about: AboutWindow,
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
    }
}
