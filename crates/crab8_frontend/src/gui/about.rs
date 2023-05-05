use egui::{Context, Window};

#[derive(Default)]
pub struct AboutWindow {
    pub open: bool,
}

impl AboutWindow {
    pub fn render(&mut self, context: &Context) {
        Window::new("About")
            .open(&mut self.open)
            .show(context, |ui| {
                // TODO: Add logo
                ui.heading("CRAB-8");

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
