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
                if ui.button("Reset").clicked() {
                    crab8.reload();
                }

                if ui.button("Stop").clicked() {
                    crab8.stop();
                }

                if crab8.is_running() {
                    if ui.button("Pause").clicked() {
                        crab8.pause();
                    }
                } else if ui.button("Play").clicked() {
                    crab8.play();
                }

                if ui.button("Step Instruction").clicked() {
                    crab8.step_instruction();
                }

                if ui.button("Step Frame").clicked() {
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
