use crate::{memory::Address, prelude::Instruction, Crab8};

/// A limit for how long to continue executing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StopCondition {
    /// Stop after a certain number of cycles.
    MaxCycles(u64),

    /// Stop after a certain number of frames.
    MaxFrames(u64),

    /// Stop when the PC reaches a specific address.
    ProgramCounter(Address),

    /// Stop when the machine is waiting for a keypress.
    PromptForInput,
}

impl StopCondition {
    /// Test to see whether this condition has been met.
    pub fn test(&self, crab8: &Crab8) -> bool {
        use StopCondition::*;

        match self {
            MaxCycles(count) => crab8.cycle_count > *count,
            MaxFrames(count) => crab8.frame_count > *count,
            ProgramCounter(address) => crab8.program_counter == *address,
            PromptForInput => matches!(
                crab8.memory.get_instruction(crab8.program_counter),
                Instruction::ReadInput(_)
            ),
        }
    }
}
