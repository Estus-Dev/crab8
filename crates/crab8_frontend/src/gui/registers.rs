use crab8::{registers::Register, Crab8};
use egui::{Context, Ui, Vec2, Window};

#[derive(Default)]
pub struct RegisterWindow {
    pub open: bool,
}

impl RegisterWindow {
    #[allow(non_snake_case)]
    pub fn render(&mut self, context: &Context, crab8: &Crab8) {
        Window::new("Registers")
            .fixed_size(Vec2::new(120.0, 150.0))
            .open(&mut self.open)
            .show(context, |ui| {
                let PC = crab8.program_counter.get();
                let I = crab8.address_register.get();
                let DT = crab8.delay.get();
                let ST = crab8.sound.get();

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("PC={PC:#05X}"));
                        ui.label(format!("I={I:#05X}"));
                    });

                    ui.vertical(|ui| {
                        ui.label(format!("DT={DT:#04X}"));
                        ui.label(format!("ST={ST:#04X}"));
                    });
                });

                ui.separator();

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
            });
    }
}

fn register_label(ui: &mut Ui, register: u16, crab8: &Crab8) {
    let register = Register::try_from(register).expect("A nibble is a valid register");
    let value = crab8.registers.get(register);

    ui.label(format!("{register:?}={value:#04X}"));
}
