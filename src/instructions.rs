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

    /// Copy a value between registers
    /// Value: 8XY0 where X is the destination and Y is the source
    Copy(Register, Register),

    /// Bitwise OR two registers, storing the result
    /// Value: 8XY1 where X is the destination and Y is the OR value
    Or(Register, Register),

    /// Bitwise AND two registers, storing the result
    /// Value: 8XY2 where X is the destination and Y is the AND value
    And(Register, Register),

    /// Bitwise XOR two registers, storing the result
    /// Value: 8XY3 where X is the destination and Y is the XOR value
    Xor(Register, Register),

    /// Add a value to the specified register and carry the result to VF
    /// Value: 8XY4 where X is the register and Y is the register to add
    AddRegister(Register, Register),

    /// Subtract a value from the specified register and flag VF on borrow
    /// Value: 8XY5 where X is the register and Y is the register to subtract
    SubtractRegister(Register, Register),

    /// Shift the value of Y right one bit and store in X, storing the shifted bit in VF
    /// Value: 8XY6 where X is the destination and Y is the value to be shifted
    ShiftRight(Register, Register),

    /// Shift the value of Y left one bit and store in X, storing the shifted bit in VF
    /// Value: 8XYE where X is the destination and Y is the value to be shifted
    ShiftLeft(Register, Register),

    /// Rather than fail parsing we'll return an invalid instruction
    Invalid(u16),

    /// This is a placeholder for as long as we don't have all the instructions in
    Unknown(u16),
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

    fn parse_register_to_register(instruction: u16) -> Instruction {
        let last_nibble = (instruction & 0x000F) as u8;

        let x =
            Register::try_from((instruction & 0x0F00) >> 8).expect("A nibble is a valid register");
        let y =
            Register::try_from((instruction & 0x00F0) >> 4).expect("A nibble is a valid register");

        match last_nibble {
            0x0 => Self::Copy(x, y),
            0x1 => Self::Or(x, y),
            0x2 => Self::And(x, y),
            0x3 => Self::Xor(x, y),
            0x4 => Self::AddRegister(x, y),
            0x5 => Self::SubtractRegister(x, y),
            0x6 => Self::ShiftRight(x, y),
            0xE => Self::ShiftLeft(x, y),
            _ => Self::Invalid(instruction),
        }
    }
}

impl From<u16> for Instruction {
    fn from(instruction: u16) -> Self {
        let first_nibble = ((instruction & 0xF000) >> 12) as u8;

        match first_nibble {
            0x6 => Self::parse_store(instruction),
            0x7 => Self::parse_add(instruction),
            0x8 => Self::parse_register_to_register(instruction),
            _ => Self::Unknown(instruction),
        }
    }
}

impl Chip8 {
    pub fn exec(&mut self, instruction: impl Into<Instruction>) {
        use Instruction::*;
        use Register::*;

        match instruction.into() {
            Store(register, value) => self.registers.set(register, value),

            Add(register, value) => self
                .registers
                .set(register, self.registers.get(register).wrapping_add(value)),

            Copy(to, from) => self.registers.set(to, self.registers.get(from)),

            Or(to, with) => self
                .registers
                .set(to, self.registers.get(to) | self.registers.get(with)),

            And(to, with) => self
                .registers
                .set(to, self.registers.get(to) & self.registers.get(with)),

            Xor(to, with) => self
                .registers
                .set(to, self.registers.get(to) ^ self.registers.get(with)),

            AddRegister(to, with) => {
                let to_value = self.registers.get(to);
                let with_value = self.registers.get(with);
                let total = to_value.wrapping_add(with_value);
                let mut wrap = 0x00;

                if total < to_value || total < with_value {
                    wrap = 0x01;
                }

                self.registers.set(to, total);
                self.registers.set(VF, wrap);
            }

            SubtractRegister(from, with) => {
                let from_value = self.registers.get(from);
                let with_value = self.registers.get(with);
                let total = from_value.wrapping_sub(with_value);
                let mut wrap = 0x00;

                if total > from_value {
                    wrap = 0x01;
                }

                self.registers.set(from, total);
                self.registers.set(VF, wrap);
            }

            ShiftRight(to, from) => {
                let value = self.registers.get(from);

                self.registers.set(to, value >> 1);
                self.registers.set(VF, value & 0b00000001);
            }

            ShiftLeft(to, from) => {
                let value = self.registers.get(from);

                self.registers.set(to, value << 1);
                self.registers.set(VF, (value & 0b10000000) >> 7);
            }

            Invalid(instruction) => panic!("Invalid instruction {instruction} executed!"),

            Unknown(instruction) => panic!("Unknown instruction {instruction} executed!"),
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

        assert_eq!(chip8.registers, 0x00000000000000000000000000000000.into());

        chip8.exec(Store(V0, 0xFF));

        assert_eq!(chip8.registers, 0xFF000000000000000000000000000000.into());

        chip8.exec(Store(V5, 0x24));

        assert_eq!(chip8.registers, 0xFF000000002400000000000000000000.into());

        chip8.exec(Store(V5, 0x00));

        assert_eq!(chip8.registers, 0xFF000000000000000000000000000000.into());
    }

    #[test]
    fn test_add() {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.registers, 0x00000000000000000000000000000000.into());

        chip8.exec(Store(V0, 0x12));

        assert_eq!(chip8.registers, 0x12000000000000000000000000000000.into());

        chip8.exec(Add(V0, 0x34));

        assert_eq!(chip8.registers, 0x46000000000000000000000000000000.into());

        chip8.exec(Add(V5, 0x47));

        assert_eq!(chip8.registers, 0x46000000004700000000000000000000.into());

        chip8.exec(Store(V2, 0xAA));

        assert_eq!(chip8.registers, 0x4600AA00004700000000000000000000.into());

        chip8.exec(Add(V2, 0x66));

        assert_eq!(chip8.registers, 0x46001000004700000000000000000000.into());
    }

    #[test]
    fn test_copy() {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.registers, 0x00000000000000000000000000000000.into());

        chip8.exec(Store(V0, 0x12));

        assert_eq!(chip8.registers, 0x12000000000000000000000000000000.into());

        chip8.exec(Copy(V1, V0));

        assert_eq!(chip8.registers, 0x12120000000000000000000000000000.into());

        chip8.exec(Store(V1, 0x63));

        assert_eq!(chip8.registers, 0x12630000000000000000000000000000.into());

        chip8.exec(Copy(V8, V1));

        assert_eq!(chip8.registers, 0x12630000000000006300000000000000.into());
    }

    #[test]
    fn test_or() {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.registers, 0x00000000000000000000000000000000.into());

        chip8.exec(Store(V0, 0b00100100));

        assert_eq!(chip8.registers.get(V0), 0b00100100);

        chip8.exec(Store(V1, 0b00111000));
        chip8.exec(Or(V0, V1));

        assert_eq!(chip8.registers.get(V0), 0b00111100);
        assert_eq!(chip8.registers.get(V1), 0b00111000);

        chip8.exec(Store(V6, 0b00000000));
        chip8.exec(Or(V6, V1));

        assert_eq!(chip8.registers.get(V6), 0b00111000);
        assert_eq!(chip8.registers.get(V1), 0b00111000);
    }

    #[test]
    fn test_and() {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.registers, 0x00000000000000000000000000000000.into());

        chip8.exec(Store(V0, 0b00100100));
        chip8.exec(Store(V1, 0b00111000));

        assert_eq!(chip8.registers.get(V0), 0b00100100);
        assert_eq!(chip8.registers.get(V1), 0b00111000);

        chip8.exec(And(V0, V1));

        assert_eq!(chip8.registers.get(V0), 0b00100000);
        assert_eq!(chip8.registers.get(V1), 0b00111000);

        chip8.exec(Store(V6, 0b00000000));

        chip8.exec(Or(V6, V1));

        assert_eq!(chip8.registers.get(V6), 0b00111000);
        assert_eq!(chip8.registers.get(V1), 0b00111000);
    }

    #[test]
    fn test_xor() {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.registers, 0x00000000000000000000000000000000.into());

        chip8.exec(Store(V0, 0b00100100));
        chip8.exec(Store(V1, 0b00111000));

        assert_eq!(chip8.registers.get(V0), 0b00100100);
        assert_eq!(chip8.registers.get(V1), 0b00111000);

        chip8.exec(Xor(V0, V1));

        assert_eq!(chip8.registers.get(V0), 0b00011100);
        assert_eq!(chip8.registers.get(V1), 0b00111000);

        chip8.exec(Store(V6, 0b00000000));

        chip8.exec(Xor(V6, V1));

        assert_eq!(chip8.registers.get(V6), 0b00111000);
        assert_eq!(chip8.registers.get(V1), 0b00111000);
    }

    #[test]
    fn test_add_register_with_carry() {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.registers, 0x00000000000000000000000000000000.into());

        chip8.exec(Store(V0, 0x12));
        chip8.exec(Store(V3, 0x89));

        assert_eq!(chip8.registers, 0x12000089000000000000000000000000.into());

        chip8.exec(AddRegister(V3, V0));

        assert_eq!(chip8.registers, 0x1200009B000000000000000000000000.into());

        chip8.exec(AddRegister(V0, V3));

        assert_eq!(chip8.registers, 0xAD00009B000000000000000000000000.into());

        chip8.exec(AddRegister(V0, V3));

        assert_eq!(chip8.registers, 0x4800009B000000000000000000000001.into());
    }

    #[test]
    fn test_subtract_register_with_carry() {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.registers, 0x00000000000000000000000000000000.into());

        chip8.exec(Store(V0, 0x12));
        chip8.exec(Store(V3, 0x89));

        assert_eq!(chip8.registers, 0x12000089000000000000000000000000.into());

        chip8.exec(SubtractRegister(V3, V0));

        assert_eq!(chip8.registers, 0x12000077000000000000000000000000.into());

        chip8.exec(SubtractRegister(V0, V3));

        assert_eq!(chip8.registers, 0x9B000077000000000000000000000001.into());

        chip8.exec(SubtractRegister(V0, V3));

        assert_eq!(chip8.registers, 0x24000077000000000000000000000000.into());
    }

    #[test]
    fn test_shift_right() {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.registers, 0x00000000000000000000000000000000.into());

        chip8.exec(Store(V0, 0b00100100));

        assert_eq!(chip8.registers.get(V0), 0b00100100);

        chip8.exec(ShiftRight(V1, V0));

        assert_eq!(chip8.registers.get(V0), 0b00100100);
        assert_eq!(chip8.registers.get(V1), 0b00010010);
        assert_eq!(chip8.registers.get(VF), 0x00);

        chip8.exec(ShiftRight(V2, V1));

        assert_eq!(chip8.registers.get(V1), 0b00010010);
        assert_eq!(chip8.registers.get(V2), 0b00001001);
        assert_eq!(chip8.registers.get(VF), 0x00);

        chip8.exec(ShiftRight(V3, V2));

        assert_eq!(chip8.registers.get(V2), 0b00001001);
        assert_eq!(chip8.registers.get(V3), 0b00000100);
        assert_eq!(chip8.registers.get(VF), 0x01);

        chip8.exec(ShiftRight(V4, V3));

        assert_eq!(chip8.registers.get(V3), 0b00000100);
        assert_eq!(chip8.registers.get(V4), 0b00000010);
        assert_eq!(chip8.registers.get(VF), 0x00);
    }

    #[test]
    fn test_shift_left() {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.registers, 0x00000000000000000000000000000000.into());

        chip8.exec(Store(V0, 0b00100100));

        assert_eq!(chip8.registers.get(V0), 0b00100100);

        chip8.exec(ShiftLeft(V1, V0));

        assert_eq!(chip8.registers.get(V0), 0b00100100);
        assert_eq!(chip8.registers.get(V1), 0b01001000);
        assert_eq!(chip8.registers.get(VF), 0x00);

        chip8.exec(ShiftLeft(V2, V1));

        assert_eq!(chip8.registers.get(V1), 0b01001000);
        assert_eq!(chip8.registers.get(V2), 0b10010000);
        assert_eq!(chip8.registers.get(VF), 0x00);

        chip8.exec(ShiftLeft(V3, V2));

        assert_eq!(chip8.registers.get(V2), 0b10010000);
        assert_eq!(chip8.registers.get(V3), 0b00100000);
        assert_eq!(chip8.registers.get(VF), 0x01);

        chip8.exec(ShiftLeft(V4, V3));

        assert_eq!(chip8.registers.get(V3), 0b00100000);
        assert_eq!(chip8.registers.get(V4), 0b01000000);
        assert_eq!(chip8.registers.get(VF), 0x00);
    }

    #[test]
    fn test_instruction_from() {
        let cases = [
            (0x64AC, Store(V4, 0xAC)),
            (0x6000, Store(V0, 0x00)),
            (0x6123, Store(V1, 0x23)),
            (0x6FFF, Store(VF, 0xFF)),
            (0x74AC, Add(V4, 0xAC)),
            (0x7000, Add(V0, 0x00)),
            (0x7123, Add(V1, 0x23)),
            (0x7FFF, Add(VF, 0xFF)),
            (0x84A0, Copy(V4, VA)),
            (0x8000, Copy(V0, V0)),
            (0x8120, Copy(V1, V2)),
            (0x8FF0, Copy(VF, VF)),
            (0x8AD1, Or(VA, VD)),
            (0x8401, Or(V4, V0)),
            (0x8E12, And(VE, V1)),
            (0x86B2, And(V6, VB)),
            (0x8933, Xor(V9, V3)),
            (0x8AF3, Xor(VA, VF)),
            (0x8DE4, AddRegister(VD, VE)),
            (0x8C44, AddRegister(VC, V4)),
            (0x8E05, SubtractRegister(VE, V0)),
            (0x8725, SubtractRegister(V7, V2)),
            (0x8126, ShiftRight(V1, V2)),
            (0x8546, ShiftRight(V5, V4)),
            (0x89FE, ShiftLeft(V9, VF)),
            (0x8CAE, ShiftLeft(VC, VA)),
        ];

        for case in cases {
            assert_eq!(Instruction::from(case.0), case.1);
        }
    }
}
