use std::cmp::max;

use crab8::{prelude::Instruction, Crab8};
use egui::{Context, Grid, RichText, Vec2, Window};
use itertools::Itertools;

#[derive(Default)]
pub struct MemoryWindow {
    pub open: bool,
}

impl MemoryWindow {
    pub fn render(&mut self, context: &Context, crab8: &Crab8) {
        Window::new("Memory")
            .fixed_size(Vec2::new(250.0, 150.0))
            .open(&mut self.open)
            .show(context, |ui| {
                Grid::new("Instruction Table").show(ui, |ui| {
                    let first_addr = usize::from(crab8.program_counter) as isize;
                    let first_addr = max(0, first_addr - 10) as usize;
                    for (addr, bytes) in crab8
                        .memory
                        .iter()
                        .skip(first_addr)
                        .chunks(2)
                        .into_iter()
                        .map(|mut chunk| {
                            let left = chunk.next().unwrap();
                            let right = chunk.next().unwrap();
                            (left.0, [left.1, right.1])
                        })
                        .take(20)
                    {
                        let instr: Instruction = bytes.into();

                        let mut addr_text = RichText::new(format!("{addr:04X}:"));
                        let mut byte_text =
                            RichText::new(format!("{:02X} {:02X}", bytes[0], bytes[1]));
                        let mut instr_text = RichText::new(format!("{instr}"));

                        if crab8.program_counter == addr {
                            addr_text = addr_text.strong();
                            byte_text = byte_text.strong();
                            instr_text = instr_text.strong();
                        }

                        ui.label(addr_text);
                        ui.label(byte_text);
                        ui.label(instr_text);
                        ui.end_row();
                    }
                });
            });
    }
}
