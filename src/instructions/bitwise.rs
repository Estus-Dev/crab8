use crate::{
    registers::Register::{self, *},
    Crab8,
};

impl Crab8 {
    pub fn exec_or(&mut self, register: Register, other: Register) {
        let starting_value = self.registers.get(register);
        let value = self.registers.get(other);
        let result = starting_value | value;

        self.registers.set(register, result);
    }

    pub fn exec_and(&mut self, register: Register, other: Register) {
        let starting_value = self.registers.get(register);
        let value = self.registers.get(other);
        let result = starting_value & value;

        self.registers.set(register, result);
    }

    pub fn exec_xor(&mut self, register: Register, other: Register) {
        let starting_value = self.registers.get(register);
        let value = self.registers.get(other);
        let result = starting_value ^ value;

        self.registers.set(register, result);
    }

    pub fn exec_shift_right(&mut self, register: Register, other: Register) {
        let value = self.registers.get(other);
        let result = value >> 1;
        let least_significant_bit = value & 0b00000001;

        self.registers.set(register, result);
        self.registers.set(VF, least_significant_bit);
    }

    pub fn exec_shift_left(&mut self, register: Register, other: Register) {
        let value = self.registers.get(other);
        let result = value << 1;
        let most_significant_bit = (value & 0b10000000) >> 7;

        self.registers.set(register, result);
        self.registers.set(VF, most_significant_bit);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::Instruction::*;

    #[test]
    fn or() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        crab8.exec(Store(V0, 0b00100100));

        assert_eq!(crab8.registers.get(V0), 0b00100100);

        crab8.exec(Store(V1, 0b00111000));
        crab8.exec(Or(V0, V1));

        assert_eq!(crab8.registers.get(V0), 0b00111100);
        assert_eq!(crab8.registers.get(V1), 0b00111000);

        crab8.exec(Store(V6, 0b00000000));
        crab8.exec(Or(V6, V1));

        assert_eq!(crab8.registers.get(V6), 0b00111000);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
    }

    #[test]
    fn and() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        crab8.exec(Store(V0, 0b00100100));
        crab8.exec(Store(V1, 0b00111000));

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b00111000);

        crab8.exec(And(V0, V1));

        assert_eq!(crab8.registers.get(V0), 0b00100000);
        assert_eq!(crab8.registers.get(V1), 0b00111000);

        crab8.exec(Store(V6, 0b00000000));

        crab8.exec(Or(V6, V1));

        assert_eq!(crab8.registers.get(V6), 0b00111000);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
    }

    #[test]
    fn xor() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        crab8.exec(Store(V0, 0b00100100));
        crab8.exec(Store(V1, 0b00111000));

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b00111000);

        crab8.exec(Xor(V0, V1));

        assert_eq!(crab8.registers.get(V0), 0b00011100);
        assert_eq!(crab8.registers.get(V1), 0b00111000);

        crab8.exec(Store(V6, 0b00000000));

        crab8.exec(Xor(V6, V1));

        assert_eq!(crab8.registers.get(V6), 0b00111000);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
    }

    #[test]
    fn shift_right() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        crab8.exec(Store(V0, 0b00100100));

        assert_eq!(crab8.registers.get(V0), 0b00100100);

        crab8.exec(ShiftRight(V1, V0));

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b00010010);
        assert_eq!(crab8.registers.get(VF), 0x00);

        crab8.exec(ShiftRight(V2, V1));

        assert_eq!(crab8.registers.get(V1), 0b00010010);
        assert_eq!(crab8.registers.get(V2), 0b00001001);
        assert_eq!(crab8.registers.get(VF), 0x00);

        crab8.exec(ShiftRight(V3, V2));

        assert_eq!(crab8.registers.get(V2), 0b00001001);
        assert_eq!(crab8.registers.get(V3), 0b00000100);
        assert_eq!(crab8.registers.get(VF), 0x01);

        crab8.exec(ShiftRight(V4, V3));

        assert_eq!(crab8.registers.get(V3), 0b00000100);
        assert_eq!(crab8.registers.get(V4), 0b00000010);
        assert_eq!(crab8.registers.get(VF), 0x00);
    }

    #[test]
    fn shift_left() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        crab8.exec(Store(V0, 0b00100100));

        assert_eq!(crab8.registers.get(V0), 0b00100100);

        crab8.exec(ShiftLeft(V1, V0));

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b01001000);
        assert_eq!(crab8.registers.get(VF), 0x00);

        crab8.exec(ShiftLeft(V2, V1));

        assert_eq!(crab8.registers.get(V1), 0b01001000);
        assert_eq!(crab8.registers.get(V2), 0b10010000);
        assert_eq!(crab8.registers.get(VF), 0x00);

        crab8.exec(ShiftLeft(V3, V2));

        assert_eq!(crab8.registers.get(V2), 0b10010000);
        assert_eq!(crab8.registers.get(V3), 0b00100000);
        assert_eq!(crab8.registers.get(VF), 0x01);

        crab8.exec(ShiftLeft(V4, V3));

        assert_eq!(crab8.registers.get(V3), 0b00100000);
        assert_eq!(crab8.registers.get(V4), 0b01000000);
        assert_eq!(crab8.registers.get(VF), 0x00);
    }
}
