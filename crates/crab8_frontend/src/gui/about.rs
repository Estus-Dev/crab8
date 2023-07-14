use egui::{Context, TextureHandle, TextureOptions, Window};

use super::images;

pub struct AboutWindow {
    pub open: bool,
    pub logo: TextureHandle,
}

impl AboutWindow {
    pub fn new(context: &Context) -> Self {
        let handle = images::load_egui_image(images::LOGO).expect("Logo is built-in");

        Self {
            open: false,
            logo: context.load_texture("crab8-logo", handle, TextureOptions::LINEAR),
        }
    }

    pub fn render(&mut self, context: &Context) {
        Window::new("About")
            .open(&mut self.open)
            .show(context, |ui| {
                ui.vertical_centered(|ui| {
                    ui.image(self.logo.id(), (128.0, 128.0));
                    ui.heading("CRAB-8");
                });

                ui.add_space(20.0);

                ui.label("A CHIP-8 emulator written in Rust for the fun of it.");
                ui.label("Available on desktop and the web.");

                ui.add_space(20.0);

                ui.separator();

                ui.add_space(5.0);

                ui.vertical_centered(|ui| {
                    ui.horizontal_top(|ui| {
                        ui.hyperlink_to("Github", "https://github.com/Estus-Dev/crab8");
                        ui.hyperlink_to("Web", "https://crab8.estus.dev");
                    });
                });
            });
    }
}
