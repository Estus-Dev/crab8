use crate::{memory::Address, Crab8};

/// A limit for how long to continue executing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StopCondition {
    /// Stop after a certain number of cycles.
    MaxCycles(u64),

    /// Stop after a certain number of frames.
    MaxFrames(u64),

    /// Stop when the PC reaches a specific address.
    ProgramCounter(Address),
}

impl StopCondition {
    /// Test to see whether this condition has been met.
    pub fn test(&self, crab8: &Crab8) -> bool {
        use StopCondition::*;

        match self {
            MaxCycles(count) => crab8.cycle_count > *count,
            MaxFrames(count) => crab8.frame_count > *count,
            ProgramCounter(address) => crab8.program_counter == *address,
        }
    }
}
