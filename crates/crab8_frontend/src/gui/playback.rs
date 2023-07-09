use crab8::Crab8;
use egui::{Context, Vec2, Window};

#[derive(Default)]
pub struct PlaybackWindow {
    pub open: bool,
}

impl PlaybackWindow {
    #[allow(non_snake_case)]
    pub fn render(&mut self, context: &Context, crab8: &mut Crab8) {
        Window::new("Playback")
            .fixed_size(Vec2::new(120.0, 150.0))
            .open(&mut self.open)
            .show(context, |ui| {
                let stopped = crab8.is_stopped();

                if ui.button("Reset").clicked() {
                    crab8.reload();

                    if stopped {
                        crab8.pause();
                    }
                }

                let stop_button = egui::Button::new("Stop");
                if ui.add_enabled(!stopped, stop_button).clicked() {
                    crab8.stop();
                }

                if crab8.is_running() {
                    if ui.button("Pause").clicked() {
                        crab8.pause();
                    }
                } else if ui.button("Play").clicked() {
                    crab8.play();
                }

                let step_i_button = egui::Button::new("Step Instruction");
                if ui.add_enabled(!stopped, step_i_button).clicked() {
                    crab8.step_instruction();
                }

                let step_f_button = egui::Button::new("Step Frame");
                if ui.add_enabled(!stopped, step_f_button).clicked() {
                    crab8.step_frame();
                }

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label(format!("Frames: {}", crab8.frame_count));
                    ui.spacing();
                    ui.label(format!("Cycles: {}", crab8.cycle_count));
                })
            });
    }
}
