use core::panic;
use rand::random;

use crate::prelude::*;

/// Chip-8 instructions are 32-bit values that may contain data
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    /// Jump moves the instruction pointer to the specified Address
    /// Value: 1NNN where NNN is the address
    Jump(Address),

    /// Call moves the PC to the specified address, pushing the current PC to the stack.
    /// Value: 2NNN where NNN is the address
    Call(Address),

    /// Skip the next instruction if the current value of the register is equal to this value.
    /// Value: 3XNN where X is the register and NN is the value to compare
    IfNot(Register, u8),

    /// Skip the next instruction if the current value of the register is not equal to this value.
    /// Value: 4XNN where X is the register and NN is the value to store
    If(Register, u8),

    /// Skip the next instruction if the current values of both registers are equal.
    /// Value: 5XY0 where X and Y are the registers to compare
    IfNotRegisters(Register, Register),

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

    /// Skip the next instruction if the current values of both registers are not equal.
    /// Value: 9XY0 where X and Y are the registers to compare
    IfRegisters(Register, Register),

    /// Store a memory address in I.
    /// Value: ANNN where NNN is the address to store.
    StoreAddress(Address),

    /// Jump moves the instruction pointer to the specified Address offset by the value of V0.
    /// Value: BNNN where NNN is the address
    JumpOffset(Address),

    /// Generate a random value in the specified register with a given bitmask
    /// Value: CXNN where X is the register and NN is the bitmask to apply
    Rand(Register, u8),

    /// Skip the next instruction if the key stored in the specified register is pressed.
    /// Value: EX9E where X is the register
    IfNotPressed(Register),

    /// Skip the next instruction if the key stored in the specified register is not pressed.
    /// Value: EXA1 where X is the register
    IfPressed(Register),

    /// Read the value of the delay timer to the specified register
    /// Value: FX07 where X is the register
    ReadDelay(Register),

    /// Set the delay timer to the value of the specified register
    /// Value: FX15 where X is the register
    SetDelay(Register),

    /// Set the sound timer to the value of the specified register
    /// Value: FX18 where X is the register
    SetSound(Register),

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
            0x2 => Self::Call(address),
            0x3 => Self::IfNot(x, value),
            0x4 => Self::If(x, value),
            0x5 if sub_operator == 0 => Self::IfNotRegisters(x, y),
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

            0x9 if sub_operator == 0x0 => Self::IfRegisters(x, y),
            0xA => Self::StoreAddress(address),
            0xB => Self::JumpOffset(address),
            0xC => Self::Rand(x, value),
            0xE if value == 0x9E => Self::IfNotPressed(x),
            0xE if value == 0xA1 => Self::IfPressed(x),

            0xF => match value {
                0x07 => Self::ReadDelay(x),
                0x15 => Self::SetDelay(x),
                0x18 => Self::SetSound(x),
                _ => Self::Invalid(instruction),
            },

            _ => Self::Unknown(instruction),
        }
    }
}

impl Chip8 {
    pub fn exec(&mut self, instruction: impl Into<Instruction>) {
        use Instruction::*;

        match instruction.into() {
            Jump(address) => self.exec_jump(address),
            Call(address) => self.exec_call(address),
            IfNot(register, value) => self.exec_if_not(register, value),
            If(register, value) => self.exec_if(register, value),
            IfNotRegisters(register, other) => self.exec_if_not_registers(register, other),
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
            IfRegisters(register, other) => self.exec_if_registers(register, other),
            StoreAddress(address) => self.exec_store_address(address),
            JumpOffset(address) => self.exec_jump_offset(address),
            Rand(register, bitmask) => self.exec_rand(register, bitmask),
            IfNotPressed(register) => self.exec_if_not_pressed(register),
            IfPressed(register) => self.exec_if_pressed(register),
            ReadDelay(register) => self.exec_read_delay(register),
            SetDelay(register) => self.exec_set_delay(register),
            SetSound(register) => self.exec_set_sound(register),
            Invalid(instruction) => panic!("Invalid instruction {instruction} executed!"),
            Unknown(instruction) => panic!("Unknown instruction {instruction} executed!"),
        }
    }

    fn exec_jump(&mut self, address: Address) {
        self.program_counter.set(address);
    }

    fn exec_call(&mut self, address: Address) {
        self.stack
            .push(self.program_counter)
            .expect("Stack Overflow");
        self.program_counter.set(address);
    }

    fn exec_if_not(&mut self, register: Register, value: u8) {
        let current_value = self.registers.get(register);

        if current_value == value {
            self.program_counter
                .set(self.program_counter.wrapping_add(1));
        }
    }

    fn exec_if(&mut self, register: Register, value: u8) {
        let current_value = self.registers.get(register);

        if current_value != value {
            self.program_counter
                .set(self.program_counter.wrapping_add(1));
        }
    }

    fn exec_if_not_registers(&mut self, register: Register, other: Register) {
        let current_value = self.registers.get(register);
        let value = self.registers.get(other);

        if current_value == value {
            self.program_counter
                .set(self.program_counter.wrapping_add(1));
        }
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

    fn exec_if_registers(&mut self, register: Register, other: Register) {
        let current_value = self.registers.get(register);
        let value = self.registers.get(other);

        if current_value != value {
            self.program_counter
                .set(self.program_counter.wrapping_add(1));
        }
    }

    fn exec_store_address(&mut self, address: Address) {
        self.address_register.set(address);
    }

    fn exec_jump_offset(&mut self, address: Address) {
        let offset = self.registers.get(V0);
        // UNDEFINED BEHAVIOR: I'm choosing to implement overflow by wrapping.
        let result = address.wrapping_add(offset);

        self.program_counter.set(result);
    }

    fn exec_rand(&mut self, register: Register, bitmask: u8) {
        let result = random::<u8>() & bitmask;

        self.registers.set(register, result);
    }

    fn exec_if_not_pressed(&mut self, register: Register) {
        let key = self.registers.get(register);
        let pressed = if key <= 0xF {
            self.input
                .is_key_pressed(key.try_into().expect("A nibble is a valid key"))
        } else {
            false
        };

        if pressed {
            self.program_counter
                .set(self.program_counter.wrapping_add(1));
        }
    }

    fn exec_if_pressed(&mut self, register: Register) {
        let key = self.registers.get(register);
        let pressed = if key <= 0xF {
            self.input
                .is_key_pressed(key.try_into().expect("A nibble is a valid key"))
        } else {
            false
        };

        if !pressed {
            self.program_counter
                .set(self.program_counter.wrapping_add(1));
        }
    }

    fn exec_read_delay(&mut self, register: Register) {
        let result = self.delay.get();

        self.registers.set(register, result);
    }

    fn exec_set_delay(&mut self, register: Register) {
        let result = self.registers.get(register);

        self.delay.set(result);
    }

    fn exec_set_sound(&mut self, register: Register) {
        let result = self.registers.get(register);

        self.sound.set(result);
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
    fn test_call() -> Result<(), ()> {
        let cases = [0x2000, 0x2234, 0x2FFF, 0x2CED, 0x22BA];

        let mut chip8 = Chip8::default();
        let mut last_pc = chip8.program_counter.get();

        for instruction in cases {
            chip8.exec(instruction);
            assert_eq!(chip8.stack.pop()?.get(), last_pc);
            last_pc = instruction & 0x0FFF;
        }

        Ok(())
    }

    #[test]
    fn test_if_not() {
        let cases = [
            (0x3000u16, 0x00u8, true),
            (0x3000, 0x01, false),
            (0x3642, 0x42, true),
            (0x3642, 0x46, false),
        ];

        let mut chip8 = Chip8::default();

        for (instruction, value, skipped) in cases {
            let register = Register::try_from((instruction & 0x0F00) >> 8) //
                .expect("A nibble is a valid register");

            let previous_pc = chip8.program_counter.get();

            chip8.exec(Store(register, value));
            chip8.exec(instruction);

            let pc = chip8.program_counter.get();

            if skipped {
                assert_eq!(pc, previous_pc + 1);
            } else {
                assert_eq!(pc, previous_pc);
            }
        }
    }

    #[test]
    fn test_if() {
        let cases = [
            (0x4000u16, 0x00u8, false),
            (0x4000, 0x01, true),
            (0x4642, 0x42, false),
            (0x4642, 0x46, true),
        ];

        let mut chip8 = Chip8::default();

        for (instruction, value, skipped) in cases {
            let register = Register::try_from((instruction & 0x0F00) >> 8) //
                .expect("A nibble is a valid register");

            let previous_pc = chip8.program_counter.get();

            chip8.exec(Store(register, value));
            chip8.exec(instruction);

            let pc = chip8.program_counter.get();

            if skipped {
                assert_eq!(pc, previous_pc + 1);
            } else {
                assert_eq!(pc, previous_pc);
            }
        }
    }

    #[test]
    fn test_if_not_register() {
        let cases = [
            (0x5000u16, 0x00u8, 0x00u8, true),
            (0x5010, 0xF5, 0xF5, true),
            (0x5010, 0xF5, 0x52, false),
            (0x5640, 0x42, 0x42, true),
            (0x5640, 0x46, 0x45, false),
        ];

        let mut chip8 = Chip8::default();

        for (instruction, x_value, y_value, skipped) in cases {
            let x = Register::try_from((instruction & 0x0F00) >> 8) //
                .expect("A nibble is a valid register");
            let y = Register::try_from((instruction & 0x00F0) >> 4) //
                .expect("A nibble is a valid register");

            let previous_pc = chip8.program_counter.get();

            chip8.exec(Store(x, x_value));
            chip8.exec(Store(y, y_value));
            chip8.exec(instruction);

            let pc = chip8.program_counter.get();

            if skipped {
                assert_eq!(pc, previous_pc + 1);
            } else {
                assert_eq!(pc, previous_pc);
            }
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
    fn test_if_register() {
        let cases = [
            (0x9000u16, 0x00u8, 0x00u8, false),
            (0x9010, 0xF5, 0xF5, false),
            (0x9010, 0xF5, 0x52, true),
            (0x9640, 0x42, 0x42, false),
            (0x9640, 0x46, 0x45, true),
        ];

        let mut chip8 = Chip8::default();

        for (instruction, x_value, y_value, skipped) in cases {
            let x = Register::try_from((instruction & 0x0F00) >> 8) //
                .expect("A nibble is a valid register");
            let y = Register::try_from((instruction & 0x00F0) >> 4) //
                .expect("A nibble is a valid register");

            let previous_pc = chip8.program_counter.get();

            chip8.exec(Store(x, x_value));
            chip8.exec(Store(y, y_value));
            chip8.exec(instruction);

            let pc = chip8.program_counter.get();

            if skipped {
                assert_eq!(pc, previous_pc + 1);
            } else {
                assert_eq!(pc, previous_pc);
            }
        }
    }

    #[test]
    fn test_store_address() -> Result<(), ()> {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.address_register, 0x000.try_into()?);

        chip8.exec(StoreAddress(0xFFF.try_into()?));

        assert_eq!(chip8.address_register, 0xFFF.try_into()?);

        chip8.exec(StoreAddress(0x032.try_into()?));

        assert_eq!(chip8.address_register, 0x032.try_into()?);

        chip8.exec(StoreAddress(0x14E.try_into()?));

        assert_eq!(chip8.address_register, 0x14E.try_into()?);

        Ok(())
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
    // TODO: Most of these tests should use some form of property-based testing
    fn test_is_not_pressed() -> Result<(), ()> {
        let mut chip8 = Chip8::default();

        for register in (0x0..=0x0F).map(|r| Register::try_from(r as usize).unwrap()) {
            for pressed_key in (0x0..=0x0F).map(|key| Key::try_from(key).unwrap()) {
                chip8.input = Input::build().set_pressed(pressed_key, true).build();

                for key in (0x0..=0x0F).map(|key| Key::try_from(key).unwrap()) {
                    let starting_pc = chip8.program_counter.get();
                    let incremented_pc = chip8.program_counter.wrapping_add(1).get();

                    chip8.registers.set(register, key as u8);
                    chip8.exec(IfNotPressed(register));

                    let pc = chip8.program_counter.get();

                    if key != pressed_key {
                        assert_eq!(pc, starting_pc);
                    } else {
                        assert_eq!(pc, incremented_pc);
                    }
                }
            }
        }

        let second_pressed_key = Key::try_from(0xC)?;

        for register in (0x0..=0x0F).map(|r| Register::try_from(r as usize).unwrap()) {
            for pressed_key in (0x0..=0x0F).map(|key| Key::try_from(key).unwrap()) {
                chip8.input = Input::build()
                    .set_pressed(pressed_key, true)
                    .set_pressed(second_pressed_key, true)
                    .build();

                for key in (0x0..=0x0F).map(|key| Key::try_from(key).unwrap()) {
                    let starting_pc = chip8.program_counter.get();
                    let incremented_pc = chip8.program_counter.wrapping_add(1).get();

                    chip8.registers.set(register, key as u8);
                    chip8.exec(IfNotPressed(register));

                    let pc = chip8.program_counter.get();

                    if key != pressed_key && key != second_pressed_key {
                        assert_eq!(pc, starting_pc);
                    } else {
                        assert_eq!(pc, incremented_pc);
                    }
                }
            }
        }

        Ok(())
    }

    #[test]
    // TODO: Most of these tests should use some form of property-based testing
    fn test_is_pressed() -> Result<(), ()> {
        let mut chip8 = Chip8::default();

        for register in (0x0..=0x0F).map(|r| Register::try_from(r as usize).unwrap()) {
            for pressed_key in (0x0..=0x0F).map(|key| Key::try_from(key).unwrap()) {
                chip8.input = Input::build().set_pressed(pressed_key, true).build();

                for key in (0x0..=0x0F).map(|key| Key::try_from(key).unwrap()) {
                    let starting_pc = chip8.program_counter.get();
                    let incremented_pc = chip8.program_counter.wrapping_add(1).get();

                    chip8.registers.set(register, key as u8);
                    chip8.exec(IfPressed(register));

                    let pc = chip8.program_counter.get();

                    if key == pressed_key {
                        assert_eq!(pc, starting_pc);
                    } else {
                        assert_eq!(pc, incremented_pc);
                    }
                }
            }
        }

        let second_pressed_key = Key::try_from(0xC)?;

        for register in (0x0..=0x0F).map(|r| Register::try_from(r as usize).unwrap()) {
            for pressed_key in (0x0..=0x0F).map(|key| Key::try_from(key).unwrap()) {
                chip8.input = Input::build()
                    .set_pressed(pressed_key, true)
                    .set_pressed(second_pressed_key, true)
                    .build();

                for key in (0x0..=0x0F).map(|key| Key::try_from(key).unwrap()) {
                    let starting_pc = chip8.program_counter.get();
                    let incremented_pc = chip8.program_counter.wrapping_add(1).get();

                    chip8.registers.set(register, key as u8);
                    chip8.exec(IfPressed(register));

                    let pc = chip8.program_counter.get();

                    if key == pressed_key || key == second_pressed_key {
                        assert_eq!(pc, starting_pc);
                    } else {
                        assert_eq!(pc, incremented_pc);
                    }
                }
            }
        }

        Ok(())
    }

    #[test]
    fn test_rand() {
        // TODO: I don't know how I want to approach testing this.
        // The bitmask needs to be tested too.
    }

    #[test]
    fn test_delay_timer() {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.delay.get(), 0x00);

        chip8.exec(Store(V5, 0x14));
        chip8.exec(SetDelay(V5));

        assert_eq!(chip8.delay.get(), 0x14);

        chip8.exec(ReadDelay(V8));

        assert_eq!(chip8.registers.get(V8), 0x14);

        chip8.tick();

        assert_eq!(chip8.delay.get(), 0x13);

        chip8.exec(ReadDelay(V8));

        assert_eq!(chip8.registers.get(V8), 0x13);

        chip8.exec(Store(V0, 0xFF));
        chip8.exec(SetDelay(V0));
        chip8.exec(ReadDelay(VF));

        assert_eq!(chip8.registers.get(VF), 0xFF);
    }

    #[test]
    fn test_sound_timer() {
        let mut chip8 = Chip8::default();

        assert_eq!(chip8.sound.get(), 0x00);

        chip8.exec(Store(V5, 0x14));
        chip8.exec(SetSound(V5));

        assert_eq!(chip8.sound.get(), 0x14);

        chip8.tick();

        assert_eq!(chip8.sound.get(), 0x13);

        chip8.exec(Store(V0, 0xFF));
        chip8.exec(SetSound(V0));
    }
    #[test]
    fn test_instruction_from() -> Result<(), ()> {
        let cases = [
            (0x1000, Jump(0x000.try_into()?)),
            (0x1234, Jump(0x234.try_into()?)),
            (0x1ABC, Jump(0xABC.try_into()?)),
            (0x2101, Call(0x101.try_into()?)),
            (0x242E, Call(0x42E.try_into()?)),
            (0x2C5D, Call(0xC5D.try_into()?)),
            (0x3271, IfNot(V2, 0x71)),
            (0x3EDD, IfNot(VE, 0xDD)),
            (0x4567, If(V5, 0x67)),
            (0x4712, If(V7, 0x12)),
            (0x5AD0, IfNotRegisters(VA, VD)),
            (0x5040, IfNotRegisters(V0, V4)),
            (0x5049, Unknown(0x5049)), // TODO: Should be Invalid
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
            (0x9AD0, IfRegisters(VA, VD)),
            (0x9040, IfRegisters(V0, V4)),
            (0x9049, Unknown(0x9049)), // TODO: Should be invalid
            (0xA000, StoreAddress(0x000.try_into()?)),
            (0xA123, StoreAddress(0x123.try_into()?)),
            (0xAF24, StoreAddress(0xF24.try_into()?)),
            (0xBFFF, JumpOffset(0xFFF.try_into()?)),
            (0xB631, JumpOffset(0x631.try_into()?)),
            (0xBD62, JumpOffset(0xD62.try_into()?)),
            (0xC700, Rand(V7, 0x00)),
            (0xC12F, Rand(V1, 0x2F)),
            (0xE09E, IfNotPressed(V0)),
            (0xE69E, IfNotPressed(V6)),
            (0xEA9E, IfNotPressed(VA)),
            (0xE2A1, IfPressed(V2)),
            (0xE9A1, IfPressed(V9)),
            (0xEBA1, IfPressed(VB)),
            (0xE09F, Unknown(0xE09F)), // TODO: Should be invalid
            (0xE1A2, Unknown(0xE1A2)), // TODO: Should be invalid
            (0xE200, Unknown(0xE200)), // TODO: Should be invalid
            (0xE3FF, Unknown(0xE3FF)), // TODO: Should be invalid
            (0xF507, ReadDelay(V5)),
            (0xF207, ReadDelay(V2)),
            (0xF000, Invalid(0xF000)),
            (0xF114, Invalid(0xF114)),
            (0xF115, SetDelay(V1)),
            (0xF015, SetDelay(V0)),
            (0xFA16, Invalid(0xFA16)),
            (0xFC17, Invalid(0xFC17)),
            (0xFB18, SetSound(VB)),
            (0xF618, SetSound(V6)),
        ];

        for case in cases {
            assert_eq!(Instruction::from(case.0), case.1);
        }

        Ok(())
    }
}
