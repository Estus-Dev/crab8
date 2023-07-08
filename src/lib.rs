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
use chip8_db::{Database, Metadata};
use input::InputBuilder;
use std::{fmt, fmt::Display};

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

    pub execution_state: ExecutionState,

    pub instructions_per_frame: usize,

    instructions_since_frame: usize,

    rom: Option<Vec<u8>>,

    // TODO: This should not be owned by Crab8, lazy_static/once_cell instead?
    pub database: Database,

    pub metadata: Option<Metadata>,
}

impl Crab8 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&mut self) {
        use ExecutionState::*;

        self.input = self.next_input.build();
        self.next_input = self.input.update();

        match self.execution_state {
            Running | StepFrame => {
                for _ in self.instructions_since_frame..self.instructions_per_frame {
                    self.execute_instruction();
                }

                self.tick();
            }

            StepInstruction => {
                self.execute_instruction();

                self.instructions_since_frame += 1;

                if self.instructions_since_frame >= self.instructions_per_frame {
                    self.tick();
                }
            }

            _ => (),
        }

        if matches!(self.execution_state, StepFrame | StepInstruction) {
            self.execution_state = Paused;
        }
    }

    fn execute_instruction(&mut self) {
        self.log_registers();

        let instruction = self.memory.get_instruction(self.program_counter);

        self.program_counter = self.program_counter.next_instruction();

        self.exec(instruction);
    }

    pub fn tick(&mut self) {
        self.delay.tick();
        self.sound.tick();

        self.instructions_since_frame = 0;
    }

    pub fn load(&mut self, rom: &[u8]) {
        let metadata = self.database.get_metadata(rom);

        if let Some(program) = &metadata.program {
            log::info!(r#"Loaded ROM "{}" ({})"#, program.title, metadata.hash);
        } else {
            log::warn!("Loaded unknown ROM ({})", metadata.hash);
        }

        self.reset();
        self.rom = Some(Vec::from(rom));
        self.memory.set_range(Address::initial_instruction(), rom);
        self.metadata = Some(metadata);
        self.execution_state = ExecutionState::Running;
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
        self.instructions_since_frame = 0;

        if let Some(rom) = self.rom.clone() {
            self.load(&rom);
        }
    }

    pub fn log_registers(&self) {
        // Based on wheremyfoodat's gameboy test log output.
        log::trace!(target: "execution_state",
        "{} D: {:02X?} S: {:02X?} CS: {:02X?} I: {:04X?} ({:02X?} {:02X?} {:02X?} {:02X?}) PC: {:04X?} ({:02X?} {:02X?} {:02X?} {:02X?})",
            self.registers,
            self.delay,
            self.sound,
            self.stack.len(),
            self.address_register,
            self.memory.get(self.address_register),
            self.memory.get(self.address_register.wrapping_add(1)),
            self.memory.get(self.address_register.wrapping_add(2)),
            self.memory.get(self.address_register.wrapping_add(3)),
            self.program_counter,
            self.memory.get(self.program_counter),
            self.memory.get(self.program_counter.wrapping_add(1)),
            self.memory.get(self.program_counter.wrapping_add(2)),
            self.memory.get(self.program_counter.wrapping_add(3)),
        );
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
            execution_state: Default::default(),
            instructions_per_frame: 10,
            instructions_since_frame: 0,
            rom: None,
            database: Database::new(),
            metadata: None,
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

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ExecutionState {
    #[default]
    Stopped,
    Running,
    Paused,
    StepInstruction,
    StepFrame,
}
