use crate::prelude::*;

/// Chip-8 instructions are 32-bit values that may contain data
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    /// Store a value in the specified register
    /// Value: 6XNN where X is the register and NN is the value to store
    Store(Register, u8),
}

impl Instruction {
    fn parse_store(instruction: u16) -> Instruction {
        let register =
            Register::try_from((instruction & 0x0F00) >> 8).expect("A nibble is a valid register");
        let value = (instruction & 0x00FF) as u8;

        Instruction::Store(register, value)
    }
}

impl TryFrom<u16> for Instruction {
    type Error = ();

    fn try_from(instruction: u16) -> Result<Self, Self::Error> {
        let first_nibble = ((instruction & 0xF000) >> 12) as u8;

        match first_nibble {
            6 => Ok(Self::parse_store(instruction)),
            _ => Err(()),
        }
    }
}

impl Chip8 {
    pub fn exec(&mut self, instruction: Instruction) {
        use Instruction::*;

        match instruction {
            Store(register, value) => self.registers.set(register, value),
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
    fn test_store_from() -> Result<(), ()> {
        let cases = [
            (0x64AC, Store(V4, 0xAC)),
            (0x6000, Store(V0, 0x00)),
            (0x6123, Store(V1, 0x23)),
            (0x6FFF, Store(VF, 0xFF)),
        ];

        for case in cases {
            assert_eq!(Instruction::try_from(case.0)?, case.1);
        }

        Ok(())
    }
}
