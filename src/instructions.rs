use core::panic;

use crate::prelude::*;

/// Chip-8 instructions are 32-bit values that may contain data
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    /// Jump moves the instruction pointer to the specified Address
    /// Value: 1NNN where NNN is the address
    Jump(Address),

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

    /// Subtract the value in the specified register from another register and flag VF on borrow
    /// The difference between this and SubtractRegister is the order, they go to the same register
    /// Value: 8XY7 where X is the register and Y is the register to subtract from
    SubtractFromRegister(Register, Register),

    /// Jump moves the instruction pointer to the specified Address offset by the value of V0.
    /// Value: BNNN where NNN is the address
    JumpOffset(Address),

    /// Rather than fail parsing we'll return an invalid instruction
    Invalid(u16),

    /// This is a placeholder for as long as we don't have all the instructions in
    Unknown(u16),
}

impl From<u16> for Instruction {
    fn from(instruction: u16) -> Self {
        let operator = ((instruction & 0xF000) >> 12) as u8;
        let sub_operator = (instruction & 0x000F) as u8;
        let x = Register::try_from((instruction & 0x0F00) >> 8) //
            .expect("A nibble is a valid register");
        let y = Register::try_from((instruction & 0x00F0) >> 4) //
            .expect("A nibble is a valid register");
        let value = (instruction & 0x00FF) as u8;
        let address = Address::try_from(instruction & 0x0FFF) //
            .expect("Addresses can be any value from 0x0000 to 0x0FFF");

        match operator {
            0x1 => Self::Jump(address),
            0x6 => Self::Store(x, value),
            0x7 => Self::Add(x, value),

            0x8 => match sub_operator {
                0x0 => Self::Copy(x, y),
                0x1 => Self::Or(x, y),
                0x2 => Self::And(x, y),
                0x3 => Self::Xor(x, y),
                0x4 => Self::AddRegister(x, y),
                0x5 => Self::SubtractRegister(x, y),
                0x6 => Self::ShiftRight(x, y),
                0x7 => Self::SubtractFromRegister(x, y),
                0xE => Self::ShiftLeft(x, y),
                _ => Self::Invalid(instruction),
            },

            0xB => Self::JumpOffset(address),

            _ => Self::Unknown(instruction),
        }
    }
}

impl Chip8 {
    pub fn exec(&mut self, instruction: impl Into<Instruction>) {
        use Instruction::*;

        match instruction.into() {
            Jump(address) => self.exec_jump(address),
            Store(register, value) => self.exec_store(register, value),
            Add(register, value) => self.exec_add(register, value),
            Copy(register, other) => self.exec_copy(register, other),
            Or(register, other) => self.exec_or(register, other),
            And(register, other) => self.exec_and(register, other),
            Xor(register, other) => self.exec_xor(register, other),
            AddRegister(register, other) => self.exec_add_register(register, other),
            SubtractRegister(register, other) => self.exec_subtract_register(register, other),
            ShiftRight(register, other) => self.exec_shift_right(register, other),
            SubtractFromRegister(register, other) => self.exec_sub_from_register(register, other),
            ShiftLeft(register, other) => self.exec_shift_left(register, other),
            JumpOffset(address) => self.exec_jump_offset(address),
            Invalid(instruction) => panic!("Invalid instruction {instruction} executed!"),
            Unknown(instruction) => panic!("Unknown instruction {instruction} executed!"),
        }
    }

    fn exec_jump(&mut self, address: Address) {
        self.program_counter.set(address);
    }

    fn exec_store(&mut self, register: Register, value: u8) {
        self.registers.set(register, value);
    }

    fn exec_add(&mut self, register: Register, value: u8) {
        let starting_value = self.registers.get(register);
        let result = starting_value.wrapping_add(value);

        self.registers.set(register, result);
    }

    fn exec_copy(&mut self, register: Register, other: Register) {
        let value = self.registers.get(other);

        self.registers.set(register, value);
    }

    fn exec_or(&mut self, register: Register, other: Register) {
        let starting_value = self.registers.get(register);
        let value = self.registers.get(other);
        let result = starting_value | value;

        self.registers.set(register, result);
    }

    fn exec_and(&mut self, register: Register, other: Register) {
        let starting_value = self.registers.get(register);
        let value = self.registers.get(other);
        let result = starting_value & value;

        self.registers.set(register, result);
    }

    fn exec_xor(&mut self, register: Register, other: Register) {
        let starting_value = self.registers.get(register);
        let value = self.registers.get(other);
        let result = starting_value ^ value;

        self.registers.set(register, result);
    }

    fn exec_add_register(&mut self, register: Register, other: Register) {
        let starting_value = self.registers.get(register);
        let value = self.registers.get(other);
        let result = starting_value.wrapping_add(value);
        let carry = result < starting_value || result < value;
        let carry = if carry { 0x01 } else { 0x00 };

        self.registers.set(register, result);
        self.registers.set(VF, carry);
    }

    fn exec_subtract_register(&mut self, register: Register, other: Register) {
        let starting_value = self.registers.get(register);
        let value = self.registers.get(other);
        let result = starting_value.wrapping_sub(value);
        let borrow = if result > starting_value { 0x01 } else { 0x00 };

        self.registers.set(register, result);
        self.registers.set(VF, borrow);
    }

    fn exec_shift_right(&mut self, register: Register, other: Register) {
        let value = self.registers.get(other);
        let result = value >> 1;
        let least_significant_bit = value & 0b00000001;

        self.registers.set(register, result);
        self.registers.set(VF, least_significant_bit);
    }

    fn exec_sub_from_register(&mut self, register: Register, other: Register) {
        let starting_value = self.registers.get(other);
        let value = self.registers.get(register);
        let result = starting_value.wrapping_sub(value);
        let borrow = if result > starting_value { 0x01 } else { 0x00 };

        self.registers.set(register, result);
        self.registers.set(VF, borrow);
    }

    fn exec_shift_left(&mut self, register: Register, other: Register) {
        let value = self.registers.get(other);
        let result = value << 1;
        let most_significant_bit = (value & 0b10000000) >> 7;

        self.registers.set(register, result);
        self.registers.set(VF, most_significant_bit);
    }

    fn exec_jump_offset(&mut self, address: Address) {
        let offset = self.registers.get(V0);
        // UNDEFINED BEHAVIOR: I'm choosing to implement overflow by wrapping.
        let result = address.wrapping_add(offset);

        self.program_counter.set(result);
    }
}

#[cfg(test)]
mod test {
    use super::Instruction::*;
    use crate::prelude::*;

    #[test]
    fn test_jump() {
        let cases = [0x1000, 0x1234, 0x1FFF, 0x1CED, 0x12BA];

        let mut chip8 = Chip8::default();

        assert_eq!(chip8.program_counter.get(), 0x200);

        for instruction in cases {
            chip8.exec(instruction);
            assert_eq!(chip8.program_counter.get(), instruction & 0x0FFF);
        }
    }

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
    fn test_sub_from_register_with_carry() {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.registers, 0x00000000000000000000000000000000.into());

        chip8.exec(Store(V0, 0x89));
        chip8.exec(Store(V3, 0x12));

        assert_eq!(chip8.registers, 0x89000012000000000000000000000000.into());

        chip8.exec(SubtractFromRegister(V3, V0));

        assert_eq!(chip8.registers, 0x89000077000000000000000000000000.into());

        chip8.exec(SubtractFromRegister(V0, V3));

        assert_eq!(chip8.registers, 0xEE000077000000000000000000000001.into());

        chip8.exec(SubtractFromRegister(V2, V0));

        assert_eq!(chip8.registers, 0xEE00EE77000000000000000000000000.into());
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
    fn test_jump_offset() {
        let cases = [
            (0xB000u16, 0x00u8, 0x000u16),
            (0xB123, 0x00, 0x123),
            (0xB123, 0x45, 0x168),
        ];

        let mut chip8 = Chip8::default();

        for (instruction, offset, expected) in cases {
            chip8.registers.set(V0, offset);
            chip8.exec(instruction);

            assert_eq!(chip8.program_counter.get(), expected);
        }
    }

    #[test]
    fn test_instruction_from() -> Result<(), ()> {
        let cases = [
            (0x1000, Jump(0x000.try_into()?)),
            (0x1234, Jump(0x234.try_into()?)),
            (0x1ABC, Jump(0xABC.try_into()?)),
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
            (0x8D57, SubtractFromRegister(VD, V5)),
            (0x8AA7, SubtractFromRegister(VA, VA)),
            (0x89FE, ShiftLeft(V9, VF)),
            (0x8CAE, ShiftLeft(VC, VA)),
        ];

        for case in cases {
            assert_eq!(Instruction::from(case.0), case.1);
        }

        Ok(())
    }
}
