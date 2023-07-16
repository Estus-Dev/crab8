pub mod character;
pub mod color;
pub mod conditions;
pub mod input;
pub mod instructions;
pub mod memory;
pub mod quirks;
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
use conditions::StopCondition;
use input::InputBuilder;
use quirks::Quirks;
use std::{fmt, fmt::Display};

const DEFAULT_TICKRATE: usize = 10;

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

    execution_state: ExecutionState,

    pub instructions_per_frame: usize,

    instructions_since_frame: usize,

    rom: Option<Vec<u8>>,

    // TODO: This should not be owned by Crab8, lazy_static/once_cell instead?
    pub database: Database,

    pub metadata: Option<Metadata>,

    // Making this public for ease of wiring to the UI, but don't mutate outside of this struct
    pub frame_count: u64,

    // Making this public for ease of wiring to the UI, but don't mutate outside of this struct
    pub cycle_count: u64,

    /// The colors specified by [chip8_db] [Metadata] for this ROM.
    pub colors: Vec<[u8; 4]>,

    pub quirks: Quirks,

    start_address: Address,
}

impl Crab8 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run_to_completion(
        &mut self,
        stop_conditions: &[StopCondition],
    ) -> Option<StopCondition> {
        self.play();

        while !self.is_stopped() && !stop_conditions.iter().any(|condition| condition.test(self)) {
            self.execute();
        }

        stop_conditions
            .iter()
            .find(|condition| condition.test(self))
            .cloned()
    }

    pub fn execute(&mut self) {
        use ExecutionState::*;

        self.input = self.next_input.build();
        self.next_input = self.input.update();

        match self.execution_state {
            Running | StepFrame => {
                for _ in self.instructions_since_frame..self.instructions_per_frame {
                    self.execute_instruction();
                    self.instructions_since_frame += 1;
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
            self.pause();
        }
    }

    fn execute_instruction(&mut self) {
        self.log_registers();

        let instruction = self.memory.get_instruction(self.program_counter);

        self.program_counter = self.program_counter.next_instruction();

        self.exec(instruction);
        self.cycle_count += 1;
    }

    pub fn tick(&mut self) {
        self.delay.tick();
        self.sound.tick();

        self.instructions_since_frame = 0;
        self.frame_count += 1;
    }

    pub fn load(&mut self, rom: &[u8]) {
        let metadata = self.database.get_metadata(rom);

        if let Some(program) = &metadata.program {
            log::info!(r#"Loaded ROM "{}" ({})"#, program.title, metadata.hash);
        } else {
            log::warn!("Loaded unknown ROM ({})", metadata.hash);
        }

        self.rom = Some(Vec::from(rom));

        self.apply_metadata(&metadata);
        self.metadata = Some(metadata);

        self.memory.set_range(self.start_address, rom);

        self.play();
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
        self.quirks = Default::default();
        self.instructions_per_frame = DEFAULT_TICKRATE;
        self.instructions_since_frame = 0;
        self.cycle_count = 0;
        self.frame_count = 0;

        self.colors.clear();
    }

    pub fn reload(&mut self) {
        self.reset();

        if let Some(rom) = self.rom.clone() {
            self.memory.set_range(self.start_address, &rom);
        }

        if let Some(metadata) = self.metadata.clone() {
            self.apply_metadata(&metadata);
        }
    }

    pub fn apply_metadata(&mut self, metadata: &Metadata) {
        if let Some(rom) = metadata.rom.as_ref() {
            self.start_address = rom
                .start_address
                .map(Address::from)
                .unwrap_or_else(Address::initial_instruction);

            self.instructions_per_frame = rom.tickrate.unwrap_or(DEFAULT_TICKRATE);

            self.colors = rom
                .colors
                .clone()
                .and_then(|colors| colors.pixels)
                .map(color::parse_colors_unchecked)
                .unwrap_or_else(Vec::new);
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self.execution_state, ExecutionState::Running)
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self.execution_state, ExecutionState::Stopped)
    }

    pub fn play(&mut self) {
        self.resume_if_stopped();

        log::debug!("Execution resumed");
        self.execution_state = ExecutionState::Running;
    }

    pub fn pause(&mut self) {
        self.resume_if_stopped();

        log::debug!("Execution paused");
        self.execution_state = ExecutionState::Paused;
    }

    pub fn stop(&mut self) {
        if matches!(self.execution_state, ExecutionState::Stopped) {
            return;
        }

        self.execution_state = ExecutionState::Stopped;
        log::info!("Execution Terminated. Final Screen:\n{}", self.screen);
    }

    pub fn step_instruction(&mut self) {
        if matches!(self.execution_state, ExecutionState::Stopped) {
            log::warn!("Attempted to step instructions after execution has stopped.");
            return;
        }

        log::debug!("Stepping instruction");
        self.execution_state = ExecutionState::StepInstruction;
    }

    pub fn step_frame(&mut self) {
        if matches!(self.execution_state, ExecutionState::Stopped) {
            log::warn!("Attempted to step frames after execution has stopped.");
            return;
        }

        log::debug!("Stepping frame");
        self.execution_state = ExecutionState::StepFrame;
    }

    fn resume_if_stopped(&mut self) {
        if matches!(self.execution_state, ExecutionState::Stopped) {
            log::info!("Beginning execution");
            self.reload();
        }
    }

    pub fn log_registers(&self) {
        log::trace!(target: "execution_state", "{}", self.dump_registers());
    }

    pub fn dump_registers(&self) -> String {
        let i_memory = format!(
            "({:02X?} {:02X?} {:02X?} {:02X?})",
            self.memory.get(self.address_register),
            self.memory.get(self.address_register.wrapping_add(1)),
            self.memory.get(self.address_register.wrapping_add(2)),
            self.memory.get(self.address_register.wrapping_add(3)),
        );

        let pc_memory = format!(
            "({:02X?} {:02X?} {:02X?} {:02X?})",
            self.memory.get(self.program_counter),
            self.memory.get(self.program_counter.wrapping_add(1)),
            self.memory.get(self.program_counter.wrapping_add(2)),
            self.memory.get(self.program_counter.wrapping_add(3)),
        );

        // Based on wheremyfoodat's gameboy test log output.
        format!(
            "{} D: {:02X?} S: {:02X?} CS: {:02X?} I: {:04X?} {} PC: {:04X?} {} - {:?}",
            self.registers,
            self.delay,
            self.sound,
            self.stack.len(),
            self.address_register,
            i_memory,
            self.program_counter,
            pc_memory,
            self.memory.get_instruction(self.program_counter),
        )
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
            screen: Screen::startup(),
            execution_state: Default::default(),
            quirks: Default::default(),
            instructions_per_frame: DEFAULT_TICKRATE,
            instructions_since_frame: 0,
            rom: None,
            database: Database::new(),
            metadata: None,
            cycle_count: 0,
            frame_count: 0,
            colors: Vec::with_capacity(16),
            start_address: Address::initial_instruction(),
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
