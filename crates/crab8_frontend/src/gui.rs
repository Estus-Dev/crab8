use egui::{menu, Context, TopBottomPanel, Window};

pub struct Gui {
    /// Whether the about window is open
    about_open: bool,
}

impl Gui {
    pub fn new() -> Self {
        Self { about_open: false }
    }

    pub fn ui(&mut self, context: &Context) {
        TopBottomPanel::top("menu_bar").show(context, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.about_open = true;
                        ui.close_menu();
                    }
                })
            })
        });

        Window::new("About")
            .open(&mut self.about_open)
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
