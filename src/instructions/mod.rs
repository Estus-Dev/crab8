mod bitwise;
mod conditional;
mod input;
mod jump;
mod math;
mod memory;
mod random;
mod registers;
mod screen;
mod timers;

use crate::prelude::*;

/// Chip-8 instructions are 32-bit values that may contain data
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    /// Clear the screen.
    /// Value: 00E0
    ClearScreen,

    /// Return from the latest subroutine.  Moves the PC to the top value of the stack.
    /// Value: 00EE
    Return,

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

    /// Draw the sprite with N rows at the current address register to the screen.
    /// Coordinates to draw are the values of the two registers X and Y
    /// Value: DXYN where X/Y are the registers with the coordinates and N is the number of rows.
    Draw(Register, Register, u8),

    /// Skip the next instruction if the key stored in the specified register is pressed.
    /// Value: EX9E where X is the register
    IfNotPressed(Register),

    /// Skip the next instruction if the key stored in the specified register is not pressed.
    /// Value: EXA1 where X is the register
    IfPressed(Register),

    /// Read the value of the delay timer to the specified register
    /// Value: FX07 where X is the register
    ReadDelay(Register),

    /// Wait until a key is pressed and store the key in the given register
    /// Value: FX0A where X is the register
    ReadInput(Register),

    /// Set the delay timer to the value of the specified register
    /// Value: FX15 where X is the register
    SetDelay(Register),

    /// Set the sound timer to the value of the specified register
    /// Value: FX18 where X is the register
    SetSound(Register),

    /// Add the value of the specified register to the address register
    /// Value: FX1E where X is the register
    AddAddress(Register),

    /// Set the address register to the sprite data for the character in the specified register.
    /// Value: FX29 where X is the register
    LoadSprite(Register),

    /// Writes the value of the specified register to the memory pointed to by the address register.
    /// The value will be written in Binary-Coded Decimal format, at 3 characters wide.
    /// https://en.wikipedia.org/wiki/Binary-coded_decimal
    /// Value: FX33 where X is the register
    WriteDecimal(Register),

    /// Writes the values of V0..=VX to the memory pointed to by the address register.
    /// The address register is incremented by X + 1.
    /// Value: FX55 where X is the final register
    Write(Register),

    /// Reads values from memory starting at the address register to fill the registers V0..=VX.
    /// The address register is incremented by X + 1.
    /// Value: FX65 where X is the final register
    Read(Register),

    /// Rather than fail parsing we'll return an invalid instruction
    NoOp(u16),
}

impl From<u16> for Instruction {
    fn from(instruction: u16) -> Self {
        let operator = ((instruction & 0xF000) >> 12) as u8;
        let sub_operator = (instruction & 0x000F) as u8;
        let x = Register::from((instruction & 0x0F00) >> 8);
        let y = Register::from((instruction & 0x00F0) >> 4);
        let value = (instruction & 0x00FF) as u8;
        let address = Address::new(instruction);

        match operator {
            0x0 if address == 0x0E0.into() => Self::ClearScreen,
            0x0 if address == 0x0EE.into() => Self::Return,
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
                _ => Self::NoOp(instruction),
            },

            0x9 if sub_operator == 0x0 => Self::IfRegisters(x, y),
            0xA => Self::StoreAddress(address),
            0xB => Self::JumpOffset(address),
            0xC => Self::Rand(x, value),
            0xD => Self::Draw(x, y, sub_operator),
            0xE if value == 0x9E => Self::IfNotPressed(x),
            0xE if value == 0xA1 => Self::IfPressed(x),

            0xF => match value {
                0x07 => Self::ReadDelay(x),
                0x0A => Self::ReadInput(x),
                0x15 => Self::SetDelay(x),
                0x18 => Self::SetSound(x),
                0x1E => Self::AddAddress(x),
                0x29 => Self::LoadSprite(x),
                0x33 => Self::WriteDecimal(x),
                0x55 => Self::Write(x),
                0x65 => Self::Read(x),
                _ => Self::NoOp(instruction),
            },

            _ => Self::NoOp(instruction),
        }
    }
}

impl Crab8 {
    pub fn exec(&mut self, instruction: impl Into<Instruction>) {
        use Instruction::*;

        match instruction.into() {
            ClearScreen => self.exec_clear_screen(),
            Return => self.exec_return(),
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
            Draw(x, y, row_count) => self.exec_draw(x, y, row_count),
            IfNotPressed(register) => self.exec_if_not_pressed(register),
            IfPressed(register) => self.exec_if_pressed(register),
            ReadDelay(register) => self.exec_read_delay(register),
            ReadInput(register) => self.exec_read_input(register),
            SetDelay(register) => self.exec_set_delay(register),
            SetSound(register) => self.exec_set_sound(register),
            AddAddress(register) => self.exec_add_address(register),
            LoadSprite(register) => self.exec_load_sprite(register),
            WriteDecimal(register) => self.exec_write_decimal(register),
            Write(register) => self.exec_write(register),
            Read(register) => self.exec_read(register),
            NoOp(instruction) => self.exec_no_op(instruction),
        }
    }

    fn exec_no_op(&mut self, _instruction: u16) {}
}

#[cfg(test)]
mod test {
    use super::Instruction::*;
    use crate::prelude::*;

    #[test]
    fn instruction_from() {
        let cases = [
            (0x00E0, ClearScreen),
            (0x00EE, Return),
            (0x1000, Jump(0x000.into())),
            (0x1234, Jump(0x234.into())),
            (0x1ABC, Jump(0xABC.into())),
            (0x2101, Call(0x101.into())),
            (0x242E, Call(0x42E.into())),
            (0x2C5D, Call(0xC5D.into())),
            (0x3271, IfNot(V2, 0x71)),
            (0x3EDD, IfNot(VE, 0xDD)),
            (0x4567, If(V5, 0x67)),
            (0x4712, If(V7, 0x12)),
            (0x5AD0, IfNotRegisters(VA, VD)),
            (0x5040, IfNotRegisters(V0, V4)),
            (0x5049, NoOp(0x5049)),
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
            (0x9049, NoOp(0x9049)),
            (0xA000, StoreAddress(0x000.into())),
            (0xA123, StoreAddress(0x123.into())),
            (0xAF24, StoreAddress(0xF24.into())),
            (0xBFFF, JumpOffset(0xFFF.into())),
            (0xB631, JumpOffset(0x631.into())),
            (0xBD62, JumpOffset(0xD62.into())),
            (0xC700, Rand(V7, 0x00)),
            (0xC12F, Rand(V1, 0x2F)),
            (0xD52B, Draw(V5, V2, 0xB)),
            (0xD1E9, Draw(V1, VE, 0x9)),
            (0xE09E, IfNotPressed(V0)),
            (0xE69E, IfNotPressed(V6)),
            (0xEA9E, IfNotPressed(VA)),
            (0xE2A1, IfPressed(V2)),
            (0xE9A1, IfPressed(V9)),
            (0xEBA1, IfPressed(VB)),
            (0xE09F, NoOp(0xE09F)),
            (0xE1A2, NoOp(0xE1A2)),
            (0xE200, NoOp(0xE200)),
            (0xE3FF, NoOp(0xE3FF)),
            (0xF507, ReadDelay(V5)),
            (0xF207, ReadDelay(V2)),
            (0xF000, NoOp(0xF000)),
            (0xF114, NoOp(0xF114)),
            (0xF115, SetDelay(V1)),
            (0xF015, SetDelay(V0)),
            (0xFA16, NoOp(0xFA16)),
            (0xFC17, NoOp(0xFC17)),
            (0xFB18, SetSound(VB)),
            (0xF618, SetSound(V6)),
            (0xF01E, AddAddress(V0)),
            (0xF41E, AddAddress(V4)),
            (0xF41F, NoOp(0xF41F)),
            (0xF129, LoadSprite(V1)),
            (0xF729, LoadSprite(V7)),
            (0xFE33, WriteDecimal(VE)),
            (0xF133, WriteDecimal(V1)),
            (0xF055, Write(V0)),
            (0xF555, Write(V5)),
            (0xF565, Read(V5)),
            (0xFA65, Read(VA)),
        ];

        for case in cases {
            assert_eq!(Instruction::from(case.0), case.1);
        }
    }
}
