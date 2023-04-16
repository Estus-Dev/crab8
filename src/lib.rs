mod registers;

pub use registers::*;
use std::{fmt, fmt::Display};

/// Chip8 represents the current state of the entire machine.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference
#[derive(Default, Debug)]
pub struct Chip8 {
    registers: [u8; 16],
}

impl Chip8 {
    /// Get the value of the selected register
    /// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#data-registers
    pub fn get_register(&self, register: Register) -> u8 {
        self.registers[register as usize]
    }

    /// Set the value of the selected register
    /// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#data-registers
    pub fn set_register(&mut self, register: Register, value: u8) {
        self.registers[register as usize] = value;
    }

    fn dump_registers(&self) -> String {
        // Get each register
        (0x00..=0x0F)
            .map(|r| r.try_into().expect("0x00..=0x0F are all the registers"))
            // Get each register with its value in the format `V0=0x00`
            .map(|r| format!("{:?}={:#04X}", r, self.get_register(r)))
            // .join(" ")
            .fold(String::new(), |rs, r| rs + &r + " ")
    }
}

impl Display for Chip8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CHIP-8")?;
        write!(f, "\tRegisters: {}", self.dump_registers())?;

        Ok(())
    }
}
