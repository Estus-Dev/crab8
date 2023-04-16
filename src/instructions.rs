use core::panic;

use crate::prelude::*;

/// Chip-8 instructions are 32-bit values that may contain data
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    /// Store a value in the specified register
    /// Value: 6XNN where X is the register and NN is the value to store
    Store(Register, u8),

    /// Add a value to the specified register
    /// Value: 7XNN where X is the register and NN is the value to add
    Add(Register, u8),

    /// We don't need to fail parsing once we have all the arguments
    /// So for now we'll return a placeholder instruction
    Unknown,
}

impl Instruction {
    fn parse_store(instruction: u16) -> Instruction {
        let register =
            Register::try_from((instruction & 0x0F00) >> 8).expect("A nibble is a valid register");
        let value = (instruction & 0x00FF) as u8;

        Instruction::Store(register, value)
    }

    fn parse_add(instruction: u16) -> Instruction {
        let register =
            Register::try_from((instruction & 0x0F00) >> 8).expect("A nibble is a valid register");
        let value = (instruction & 0x00FF) as u8;

        Instruction::Add(register, value)
    }
}

impl From<u16> for Instruction {
    fn from(instruction: u16) -> Self {
        let first_nibble = ((instruction & 0xF000) >> 12) as u8;

        match first_nibble {
            6 => Self::parse_store(instruction),
            7 => Self::parse_add(instruction),
            _ => Self::Unknown,
        }
    }
}

impl Chip8 {
    pub fn exec(&mut self, instruction: impl Into<Instruction>) {
        use Instruction::*;

        match instruction.into() {
            Store(register, value) => self.registers.set(register, value),
            Add(register, value) => self
                .registers
                .set(register, self.registers.get(register) + value),
            // While this can panic, it can only do so until we have all the instructions
            Unknown => panic!("Unknown instruction executed!"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Instruction::*;
    use super::Register::*;
    use crate::prelude::*;

    #[test]
    fn test_store() {
        let mut chip8 = Chip8::default();

        assert_eq!(
            chip8.registers,
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()
        );

        chip8.exec(Store(V0, 0xFF));

        assert_eq!(
            chip8.registers,
            [0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()
        );

        chip8.exec(Store(V5, 0x24));

        assert_eq!(
            chip8.registers,
            [0xFF, 0, 0, 0, 0, 0x24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()
        );

        chip8.exec(Store(V5, 0x00));

        assert_eq!(
            chip8.registers,
            [0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()
        );
    }

    #[test]
    fn test_add() {
        let mut chip8 = Chip8::default();

        assert_eq!(
            chip8.registers,
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()
        );

        chip8.exec(Store(V0, 0x12));

        assert_eq!(
            chip8.registers,
            [0x12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()
        );

        chip8.exec(Add(V0, 0x34));

        assert_eq!(
            chip8.registers,
            [0x46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()
        );

        chip8.exec(Add(V5, 0x47));

        assert_eq!(
            chip8.registers,
            [0x46, 0, 0, 0, 0, 0x47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into()
        );
    }

    #[test]
    fn test_store_from() -> Result<(), ()> {
        let cases = [
            (0x64AC, Store(V4, 0xAC)),
            (0x6000, Store(V0, 0x00)),
            (0x6123, Store(V1, 0x23)),
            (0x6FFF, Store(VF, 0xFF)),
        ];

        for case in cases {
            assert_eq!(Instruction::from(case.0), case.1);
        }

        Ok(())
    }

    #[test]
    fn test_add_from() -> Result<(), ()> {
        let cases = [
            (0x74AC, Add(V4, 0xAC)),
            (0x7000, Add(V0, 0x00)),
            (0x7123, Add(V1, 0x23)),
            (0x7FFF, Add(VF, 0xFF)),
        ];

        for case in cases {
            assert_eq!(Instruction::from(case.0), case.1);
        }

        Ok(())
    }
}
