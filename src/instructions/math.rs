use super::Instruction;
use crate::{
    registers::Register::{self, *},
    Crab8,
};

impl Instruction {
    pub fn add(crab8: &mut Crab8, register: Register, value: u8) {
        let starting_value = crab8.registers.get(register);
        let result = starting_value.wrapping_add(value);

        crab8.registers.set(register, result);
    }

    pub fn add_reg(crab8: &mut Crab8, register: Register, other: Register) {
        let starting_value = crab8.registers.get(register);
        let value = crab8.registers.get(other);
        let (result, carry) = starting_value.overflowing_add(value);
        let carry = if carry { 0x01 } else { 0x00 };

        crab8.registers.set(register, result);
        crab8.registers.set(VF, carry);
    }

    pub fn sub_reg(crab8: &mut Crab8, register: Register, other: Register) {
        let starting_value = crab8.registers.get(register);
        let value = crab8.registers.get(other);
        let (result, borrow) = starting_value.overflowing_sub(value);
        let no_borrow = if borrow { 0x00 } else { 0x01 };

        crab8.registers.set(register, result);
        crab8.registers.set(VF, no_borrow);
    }

    pub fn sub_from_reg(crab8: &mut Crab8, register: Register, other: Register) {
        let starting_value = crab8.registers.get(other);
        let value = crab8.registers.get(register);
        let (result, borrow) = starting_value.overflowing_sub(value);
        let no_borrow = if borrow { 0x00 } else { 0x01 };

        crab8.registers.set(register, result);
        crab8.registers.set(VF, no_borrow);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0x12);

        assert_eq!(crab8.registers, 0x12000000000000000000000000000000.into());

        Instruction::add(&mut crab8, V0, 0x34);

        assert_eq!(crab8.registers, 0x46000000000000000000000000000000.into());

        Instruction::add(&mut crab8, V5, 0x47);

        assert_eq!(crab8.registers, 0x46000000004700000000000000000000.into());

        Instruction::store(&mut crab8, V2, 0xAA);

        assert_eq!(crab8.registers, 0x4600AA00004700000000000000000000.into());

        Instruction::add(&mut crab8, V2, 0x66);

        assert_eq!(crab8.registers, 0x46001000004700000000000000000000.into());
    }

    #[test]
    fn add_register_with_carry() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0x12);
        Instruction::store(&mut crab8, V3, 0x89);

        assert_eq!(crab8.registers, 0x12000089000000000000000000000000.into());

        Instruction::add_reg(&mut crab8, V3, V0);

        assert_eq!(crab8.registers, 0x1200009B000000000000000000000000.into());

        Instruction::add_reg(&mut crab8, V0, V3);

        assert_eq!(crab8.registers, 0xAD00009B000000000000000000000000.into());

        Instruction::add_reg(&mut crab8, V0, V3);

        assert_eq!(crab8.registers, 0x4800009B000000000000000000000001.into());
    }

    #[test]
    fn subtract_register_with_carry() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0x12);
        Instruction::store(&mut crab8, V3, 0x89);

        assert_eq!(crab8.registers, 0x12000089000000000000000000000000.into());

        Instruction::sub_reg(&mut crab8, V3, V0);

        assert_eq!(crab8.registers, 0x12000077000000000000000000000001.into());

        Instruction::sub_reg(&mut crab8, V0, V3);

        assert_eq!(crab8.registers, 0x9B000077000000000000000000000000.into());

        Instruction::sub_reg(&mut crab8, V0, V3);

        assert_eq!(crab8.registers, 0x24000077000000000000000000000001.into());
    }

    #[test]
    fn sub_from_register_with_carry() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0x89);
        Instruction::store(&mut crab8, V3, 0x12);

        assert_eq!(crab8.registers, 0x89000012000000000000000000000000.into());

        Instruction::sub_from_reg(&mut crab8, V3, V0);

        assert_eq!(crab8.registers, 0x89000077000000000000000000000001.into());

        Instruction::sub_from_reg(&mut crab8, V0, V3);

        assert_eq!(crab8.registers, 0xEE000077000000000000000000000000.into());

        Instruction::sub_from_reg(&mut crab8, V2, V0);

        assert_eq!(crab8.registers, 0xEE00EE77000000000000000000000001.into());
    }
}
