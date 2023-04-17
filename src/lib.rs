mod instructions;
mod memory;
mod registers;

pub mod prelude {
    pub use crate::instructions::Instruction;
    pub use crate::memory::Address;
    pub use crate::registers::{Register, Register::*, Registers};
    pub use crate::Chip8;
}

use crate::prelude::*;
use std::{fmt, fmt::Display};

/// Chip8 represents the current state of the entire machine.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference
#[derive(Debug)]
pub struct Chip8 {
    /// The CHIP-8 has a 12-bit address register named I that points to the next instruction.
    /// Technically I is 16-bits wide but addresses above 0xF000 are inaccessible.
    /// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#address-register
    pub address_register: Address,

    /// The CHIP-8 has 16 8-bit general-purpose registers, V0-VF.
    /// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#data-registers
    pub registers: Registers,
}

impl Chip8 {}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            address_register: Address::starting_address(),
            registers: Default::default(),
        }
    }
}

impl Display for Chip8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CHIP-8")?;
        write!(f, "\tRegisters: {}", self.registers)?;

        Ok(())
    }
}
