pub mod character;
pub mod input;
pub mod instructions;
pub mod memory;
pub mod registers;
pub mod screen;
pub mod stack;
pub mod timer;

pub mod prelude {
    pub use crate::character::{Character, Character::*};
    pub use crate::input::{Input, Key, Key::*};
    pub use crate::instructions::Instruction;
    pub use crate::memory::{Address, Memory};
    pub use crate::registers::{Register, Register::*, Registers};
    pub use crate::screen::Screen;
    pub use crate::stack::Stack;
    pub use crate::timer::Timer;
    pub use crate::Crab8;
}

use crate::prelude::*;
use input::InputBuilder;
#[cfg(feature = "download")]
use reqwest::{blocking::get, Result};
use std::{fmt, fmt::Display, fs::File, io::Read, path::Path};

/// Chip8 represents the current state of the entire machine.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference
#[derive(Debug)]
pub struct Crab8 {
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

    pub next_input: InputBuilder,

    pub screen: Screen,

    pub instructions_per_frame: usize,
}

impl Crab8 {
    pub fn execute(&mut self) {
        self.input = self.next_input.build();
        self.next_input = self.input.update();

        for _ in 0..self.instructions_per_frame {
            let instruction = self.memory.get_instruction(self.program_counter);

            self.program_counter = self.program_counter.next_instruction();

            self.exec(instruction);
        }

        self.tick();
    }

    pub fn tick(&mut self) {
        self.delay.tick();
        self.sound.tick();
    }

    pub fn load(&mut self, rom: &[u8]) {
        self.reset();
        self.memory.set_range(Address::initial_instruction(), rom);
    }

    pub fn load_file<P: AsRef<Path>>(&mut self, filename: P) -> std::io::Result<()> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();

        // TODO: Check to see if it will fit in memory
        file.read_to_end(&mut buffer)?;

        self.load(&buffer);

        Ok(())
    }

    #[cfg(feature = "download")]
    pub fn download(&mut self, url: &str) -> Result<()> {
        let res = get(url)?;
        let data = res.bytes()?;
        let start = Address::initial_instruction();

        self.memory = Memory::default();
        self.memory.set_range(start, &data);

        Ok(())
    }

    pub fn reset(&mut self) {
        self.address_register = Address::default();
        self.program_counter = Address::initial_instruction();
        self.registers = Default::default();
        self.delay = Default::default();
        self.sound = Default::default();
        self.stack = Default::default();
        self.memory = Default::default();
        self.input = Default::default();
        self.next_input = Default::default();
        self.screen = Default::default();
    }
}

impl Default for Crab8 {
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
            next_input: Default::default(),
            screen: Default::default(),
            instructions_per_frame: 10,
        }
    }
}

impl Display for Crab8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CRAB-8\n")?;
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
        writeln!(f)?;
        writeln!(f, "{}", self.screen)?;

        Ok(())
    }
}
