use crab8::Crab8;
use egui::{Context, Ui, Vec2, Window};

#[derive(Default)]
pub struct RegisterWindow {
    pub open: bool,
}

impl RegisterWindow {
    #[allow(non_snake_case)]
    pub fn render(&mut self, context: &Context, crab8: &Crab8) {
        Window::new("Registers")
            .fixed_size(Vec2::new(128.0, 150.0))
            .open(&mut self.open)
            .show(context, |ui| {
                let PC = crab8.program_counter;
                let I = crab8.address_register;
                let DT = crab8.delay;
                let ST = crab8.sound;

                ui.horizontal(|ui| {
                    for column in 0u16..2 {
                        let offset = column * 0x8;

                        ui.vertical(|ui| {
                            for register in offset..(0x8 + offset) {
                                register_label(ui, register, crab8);
                            }
                        });
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!(" I={I:04?}"));
                        ui.label(format!("DT={DT:02?}"));
                        ui.label(format!("ST={ST:02?}"));
                    });

                    ui.vertical(|ui| {
                        ui.label(format!("PC={PC:04?}"));

                        for (i, address) in crab8.stack.clone().into_iter().enumerate() {
                            ui.label(format!("{i:2?}={address:04?}"));
                        }
                    });
                });
            });
    }
}

fn register_label(ui: &mut Ui, register: u16, crab8: &Crab8) {
    let register = register.into();
    let value = crab8.registers.get(register);

    ui.label(format!("{register:?}={value:#04X}"));
}
