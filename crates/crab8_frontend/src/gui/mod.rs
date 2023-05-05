mod about;
pub mod renderer;

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

    fn render(&mut self, context: &Context) {
        TopBottomPanel::top("menu_bar").show(context, |ui| {
            menu::bar(ui, |ui| {
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
