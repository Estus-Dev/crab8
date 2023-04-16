mod chip8;
mod instructions;
mod registers;

pub mod prelude {
    pub use crate::chip8::Chip8;
    pub use crate::instructions::Instruction;
    pub use crate::registers::{Register, Registers};
}
