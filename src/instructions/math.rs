use crate::{
    registers::Register::{self, *},
    Crab8,
};

impl Crab8 {
    pub fn exec_add(&mut self, register: Register, value: u8) {
        let starting_value = self.registers.get(register);
        let result = starting_value.wrapping_add(value);

        self.registers.set(register, result);
    }

    pub fn exec_add_register(&mut self, register: Register, other: Register) {
        let starting_value = self.registers.get(register);
        let value = self.registers.get(other);
        let result = starting_value.wrapping_add(value);
        let carry = result < starting_value || result < value;
        let carry = if carry { 0x01 } else { 0x00 };

        self.registers.set(register, result);
        self.registers.set(VF, carry);
    }

    pub fn exec_subtract_register(&mut self, register: Register, other: Register) {
        let starting_value = self.registers.get(register);
        let value = self.registers.get(other);
        let result = starting_value.wrapping_sub(value);
        let borrow = if result > starting_value { 0x01 } else { 0x00 };

        self.registers.set(register, result);
        self.registers.set(VF, borrow);
    }

    pub fn exec_sub_from_register(&mut self, register: Register, other: Register) {
        let starting_value = self.registers.get(other);
        let value = self.registers.get(register);
        let result = starting_value.wrapping_sub(value);
        let borrow = if result > starting_value { 0x01 } else { 0x00 };

        self.registers.set(register, result);
        self.registers.set(VF, borrow);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::Instruction::*;

    #[test]
    fn add() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        crab8.exec(Store(V0, 0x12));

        assert_eq!(crab8.registers, 0x12000000000000000000000000000000.into());

        crab8.exec(Add(V0, 0x34));

        assert_eq!(crab8.registers, 0x46000000000000000000000000000000.into());

        crab8.exec(Add(V5, 0x47));

        assert_eq!(crab8.registers, 0x46000000004700000000000000000000.into());

        crab8.exec(Store(V2, 0xAA));

        assert_eq!(crab8.registers, 0x4600AA00004700000000000000000000.into());

        crab8.exec(Add(V2, 0x66));

        assert_eq!(crab8.registers, 0x46001000004700000000000000000000.into());
    }

    #[test]
    fn add_register_with_carry() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        crab8.exec(Store(V0, 0x12));
        crab8.exec(Store(V3, 0x89));

        assert_eq!(crab8.registers, 0x12000089000000000000000000000000.into());

        crab8.exec(AddRegister(V3, V0));

        assert_eq!(crab8.registers, 0x1200009B000000000000000000000000.into());

        crab8.exec(AddRegister(V0, V3));

        assert_eq!(crab8.registers, 0xAD00009B000000000000000000000000.into());

        crab8.exec(AddRegister(V0, V3));

        assert_eq!(crab8.registers, 0x4800009B000000000000000000000001.into());
    }

    #[test]
    fn subtract_register_with_carry() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        crab8.exec(Store(V0, 0x12));
        crab8.exec(Store(V3, 0x89));

        assert_eq!(crab8.registers, 0x12000089000000000000000000000000.into());

        crab8.exec(SubtractRegister(V3, V0));

        assert_eq!(crab8.registers, 0x12000077000000000000000000000000.into());

        crab8.exec(SubtractRegister(V0, V3));

        assert_eq!(crab8.registers, 0x9B000077000000000000000000000001.into());

        crab8.exec(SubtractRegister(V0, V3));

        assert_eq!(crab8.registers, 0x24000077000000000000000000000000.into());
    }

    #[test]
    fn sub_from_register_with_carry() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        crab8.exec(Store(V0, 0x89));
        crab8.exec(Store(V3, 0x12));

        assert_eq!(crab8.registers, 0x89000012000000000000000000000000.into());

        crab8.exec(SubtractFromRegister(V3, V0));

        assert_eq!(crab8.registers, 0x89000077000000000000000000000000.into());

        crab8.exec(SubtractFromRegister(V0, V3));

        assert_eq!(crab8.registers, 0xEE000077000000000000000000000001.into());

        crab8.exec(SubtractFromRegister(V2, V0));

        assert_eq!(crab8.registers, 0xEE00EE77000000000000000000000000.into());
    }
}
