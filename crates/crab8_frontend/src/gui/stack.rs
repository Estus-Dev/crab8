use crab8::Crab8;
use egui::{Context, Vec2, Window};

#[derive(Default)]
pub struct StackWindow {
    pub open: bool,
}

impl StackWindow {
    #[allow(non_snake_case)]
    pub fn render(&mut self, context: &Context, crab8: &Crab8) {
        Window::new("Stack")
            .fixed_size(Vec2::new(120.0, 150.0))
            .open(&mut self.open)
            .show(context, |ui| {
                ui.vertical(|ui| {
                    let pc = crab8.program_counter.get();
                    ui.label(format!("PC={pc:#05X}"));
                    ui.separator();

                    for (i, address) in crab8.stack.clone().into_iter().enumerate() {
                        ui.label(format!("{i}={address}"));
                    }
                });
            });
    }
}
