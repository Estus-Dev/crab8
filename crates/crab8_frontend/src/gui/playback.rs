use crab8::{Crab8, ExecutionState};
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
                let (play_pause_label, play_pause_state) =
                    if crab8.execution_state == ExecutionState::Running {
                        ("Pause", ExecutionState::Paused)
                    } else {
                        ("Play", ExecutionState::Running)
                    };

                if ui.button("Stop").clicked() {
                    crab8.execution_state = ExecutionState::Stopped;
                }

                if ui.button(play_pause_label).clicked() {
                    crab8.execution_state = play_pause_state;
                }

                if ui.button("Step Instruction").clicked() {
                    crab8.execution_state = ExecutionState::StepInstruction;
                }

                if ui.button("Step Frame").clicked() {
                    crab8.execution_state = ExecutionState::StepFrame;
                }
            });
    }
}
