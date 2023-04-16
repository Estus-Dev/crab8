mod chip8;
mod registers;

pub mod prelude {
    pub use crate::chip8::Chip8;
    pub use crate::registers::{Register, Registers};
}
