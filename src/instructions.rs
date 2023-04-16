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
    Copy { to: Register, from: Register },

    /// Bitwise OR two registers, storing the result
    /// Value: 8XY1 where X is the destination and Y is the OR value
    Or { to: Register, with: Register },

    /// Bitwise AND two registers, storing the result
    /// Value: 8XY2 where X is the destination and Y is the AND value
    And { to: Register, with: Register },

    /// Bitwise XOR two registers, storing the result
    /// Value: 8XY3 where X is the destination and Y is the XOR value
    Xor { to: Register, with: Register },

    /// Add a value to the specified register and carry the result to VF
    /// Value: 8XY4 where X is the register and NN is the value to add
    AddRegister { to: Register, with: Register },

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
            0 => Self::Copy { to: x, from: y },
            1 => Self::Or { to: x, with: y },
            2 => Self::And { to: x, with: y },
            3 => Self::Xor { to: x, with: y },
            4 => Self::AddRegister { to: x, with: y },
            _ => Self::Invalid(instruction),
        }
    }
}

impl From<u16> for Instruction {
    fn from(instruction: u16) -> Self {
        let first_nibble = ((instruction & 0xF000) >> 12) as u8;

        match first_nibble {
            6 => Self::parse_store(instruction),
            7 => Self::parse_add(instruction),
            8 => Self::parse_register_to_register(instruction),
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

            Copy { to, from } => self.registers.set(to, self.registers.get(from)),

            Or { to, with } => self
                .registers
                .set(to, self.registers.get(to) | self.registers.get(with)),

            And { to, with } => self
                .registers
                .set(to, self.registers.get(to) & self.registers.get(with)),

            Xor { to, with } => self
                .registers
                .set(to, self.registers.get(to) ^ self.registers.get(with)),

            AddRegister { to, with } => {
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

        chip8.exec(Copy { to: V1, from: V0 });

        assert_eq!(chip8.registers, 0x12120000000000000000000000000000.into());

        chip8.exec(Store(V1, 0x63));

        assert_eq!(chip8.registers, 0x12630000000000000000000000000000.into());

        chip8.exec(Copy { to: V8, from: V1 });

        assert_eq!(chip8.registers, 0x12630000000000006300000000000000.into());
    }

    #[test]
    fn test_or() {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.registers, 0x00000000000000000000000000000000.into());

        chip8.exec(Store(V0, 0b00100100));

        assert_eq!(chip8.registers.get(V0), 0b00100100);

        chip8.exec(Store(V1, 0b00111000));
        chip8.exec(Or { to: V0, with: V1 });

        assert_eq!(chip8.registers.get(V0), 0b00111100);
        assert_eq!(chip8.registers.get(V1), 0b00111000);

        chip8.exec(Store(V6, 0b00000000));
        chip8.exec(Or { to: V6, with: V1 });

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

        chip8.exec(And { to: V0, with: V1 });

        assert_eq!(chip8.registers.get(V0), 0b00100000);
        assert_eq!(chip8.registers.get(V1), 0b00111000);

        chip8.exec(Store(V6, 0b00000000));

        chip8.exec(Or { to: V6, with: V1 });

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

        chip8.exec(Xor { to: V0, with: V1 });

        assert_eq!(chip8.registers.get(V0), 0b00011100);
        assert_eq!(chip8.registers.get(V1), 0b00111000);

        chip8.exec(Store(V6, 0b00000000));

        chip8.exec(Xor { to: V6, with: V1 });

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

        chip8.exec(AddRegister { to: V3, with: V0 });

        assert_eq!(chip8.registers, 0x1200009B000000000000000000000000.into());

        chip8.exec(AddRegister { to: V0, with: V3 });

        assert_eq!(chip8.registers, 0xAD00009B000000000000000000000000.into());

        chip8.exec(AddRegister { to: V0, with: V3 });

        assert_eq!(chip8.registers, 0x4800009B000000000000000000000001.into());
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
            (0x84A0, Copy { to: V4, from: VA }),
            (0x8000, Copy { to: V0, from: V0 }),
            (0x8120, Copy { to: V1, from: V2 }),
            (0x8FF0, Copy { to: VF, from: VF }),
            (0x8AD1, Or { to: VA, with: VD }),
            (0x8401, Or { to: V4, with: V0 }),
            (0x8E12, And { to: VE, with: V1 }),
            (0x86B2, And { to: V6, with: VB }),
            (0x8933, Xor { to: V9, with: V3 }),
            (0x8AF3, Xor { to: VA, with: VF }),
            (0x8DE4, AddRegister { to: VD, with: VE }),
            (0x8C44, AddRegister { to: VC, with: V4 }),
        ];

        for case in cases {
            assert_eq!(Instruction::from(case.0), case.1);
        }
    }
}
