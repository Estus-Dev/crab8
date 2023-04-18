mod input;
mod instructions;
mod memory;
mod registers;
mod stack;
mod timer;

pub mod prelude {
    pub use crate::input::{Input, Key, Key::*};
    pub use crate::instructions::Instruction;
    pub use crate::memory::{Address, Memory};
    pub use crate::registers::{Register, Register::*, Registers};
    pub use crate::stack::Stack;
    pub use crate::timer::Timer;
    pub use crate::Chip8;
}

use crate::prelude::*;
use std::{fmt, fmt::Display};

/// Chip8 represents the current state of the entire machine.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference
#[derive(Debug)]
pub struct Chip8 {
    /// The CHIP-8 has a 12-bit address register named I for pointing to memory.
    /// Technically I is often 16-bits wide but addresses above 0xF000 are inaccessible.
    /// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#address-register
    pub address_register: Address,

    /// The PC contains the address of the next instruction to be executed.
    /// Techinically PC is often 16-bits wide but addresses above 0xF000 are inaccessible.
    pub program_counter: Address,

    /// The CHIP-8 has 16 8-bit general-purpose registers, V0-VF.
    /// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#data-registers
    pub registers: Registers,

    pub delay: Timer,

    pub sound: Timer,

    pub stack: Stack,

    pub memory: Memory,

    pub input: Input,
}

impl Chip8 {}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            address_register: Address::default(),
            program_counter: Address::initial_instruction(),
            registers: Default::default(),
            delay: Default::default(),
            sound: Default::default(),
            stack: Default::default(),
            memory: Default::default(),
            input: Default::default(),
        }
    }
}

impl Display for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CHIP-8\n")?;
        writeln!(f, "\tAddress Register (I): {}", self.address_register)?;
        writeln!(f, "\tProgram Counter (PC): {}", self.program_counter)?;
        writeln!(f, "\tRegisters: {}", self.registers)?;
        writeln!(f, "\tDelay: {}\tSound: {}", self.delay, self.sound)?;
        writeln!(f)?;
        writeln!(f, "\tInput: {}", self.input)?;
        writeln!(f)?;
        writeln!(f, "\tStack: {}", self.stack)?;
        writeln!(f)?;
        writeln!(f, "{}", self.memory)?;

        Ok(())
    }
}
