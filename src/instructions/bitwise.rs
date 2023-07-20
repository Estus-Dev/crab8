use super::Instruction;
use crate::{
    registers::Register::{self, *},
    Crab8,
};

impl Instruction {
    pub fn or(crab8: &mut Crab8, register: Register, other: Register) {
        let starting_value = crab8.registers.get(register);
        let value = crab8.registers.get(other);
        let result = starting_value | value;

        crab8.registers.set(register, result);

        if crab8.quirks.vf_reset {
            crab8.registers.set(VF, 0);
        }
    }

    pub fn and(crab8: &mut Crab8, register: Register, other: Register) {
        let starting_value = crab8.registers.get(register);
        let value = crab8.registers.get(other);
        let result = starting_value & value;

        crab8.registers.set(register, result);

        if crab8.quirks.vf_reset {
            crab8.registers.set(VF, 0);
        }
    }

    pub fn xor(crab8: &mut Crab8, register: Register, other: Register) {
        let starting_value = crab8.registers.get(register);
        let value = crab8.registers.get(other);
        let result = starting_value ^ value;

        crab8.registers.set(register, result);

        if crab8.quirks.vf_reset {
            crab8.registers.set(VF, 0);
        }
    }

    pub fn shift_right(crab8: &mut Crab8, register: Register, other: Register) {
        let value = if crab8.quirks.shift {
            crab8.registers.get(register)
        } else {
            crab8.registers.get(other)
        };

        let result = value >> 1;
        let least_significant_bit = value & 0b00000001;

        crab8.registers.set(register, result);
        crab8.registers.set(VF, least_significant_bit);
    }

    pub fn shift_left(crab8: &mut Crab8, register: Register, other: Register) {
        let value = if crab8.quirks.shift {
            crab8.registers.get(register)
        } else {
            crab8.registers.get(other)
        };

        let result = value << 1;
        let most_significant_bit = (value & 0b10000000) >> 7;

        crab8.registers.set(register, result);
        crab8.registers.set(VF, most_significant_bit);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn or() {
        let mut crab8 = Crab8::new();

        crab8.quirks.vf_reset = false;

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0b00100100);

        assert_eq!(crab8.registers.get(V0), 0b00100100);

        Instruction::store(&mut crab8, VF, 0xBC);

        assert_eq!(crab8.registers.get(VF), 0xBC);

        Instruction::store(&mut crab8, V1, 0b00111000);
        Instruction::or(&mut crab8, V0, V1);

        assert_eq!(crab8.registers.get(V0), 0b00111100);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0xBC);

        Instruction::store(&mut crab8, V6, 0b00000000);
        Instruction::store(&mut crab8, VF, 0x1F);
        Instruction::or(&mut crab8, V6, V1);

        assert_eq!(crab8.registers.get(V6), 0b00111000);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0x1F);
    }

    #[test]
    fn or_vf_reset() {
        let mut crab8 = Crab8::new();

        crab8.quirks.vf_reset = true;

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0b00100100);

        assert_eq!(crab8.registers.get(V0), 0b00100100);

        Instruction::store(&mut crab8, VF, 0xBC);

        assert_eq!(crab8.registers.get(VF), 0xBC);

        Instruction::store(&mut crab8, V1, 0b00111000);
        Instruction::or(&mut crab8, V0, V1);

        assert_eq!(crab8.registers.get(V0), 0b00111100);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0x00);

        Instruction::store(&mut crab8, V6, 0b00000000);
        Instruction::store(&mut crab8, VF, 0x1F);
        Instruction::or(&mut crab8, V6, V1);

        assert_eq!(crab8.registers.get(V6), 0b00111000);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0x00);
    }

    #[test]
    fn and() {
        let mut crab8 = Crab8::new();

        crab8.quirks.vf_reset = false;

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0b00100100);
        Instruction::store(&mut crab8, V1, 0b00111000);
        Instruction::store(&mut crab8, VF, 0xA5);

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0xA5);

        Instruction::and(&mut crab8, V0, V1);

        assert_eq!(crab8.registers.get(V0), 0b00100000);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0xA5);

        Instruction::store(&mut crab8, V6, 0b00000000);
        Instruction::store(&mut crab8, VF, 0x01);

        Instruction::or(&mut crab8, V6, V1);

        assert_eq!(crab8.registers.get(V6), 0b00111000);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0x01);
    }

    #[test]
    fn and_vf_reset() {
        let mut crab8 = Crab8::new();

        crab8.quirks.vf_reset = true;

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0b00100100);
        Instruction::store(&mut crab8, V1, 0b00111000);
        Instruction::store(&mut crab8, VF, 0xA5);

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0xA5);

        Instruction::and(&mut crab8, V0, V1);

        assert_eq!(crab8.registers.get(V0), 0b00100000);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0x00);

        Instruction::store(&mut crab8, V6, 0b00000000);
        Instruction::store(&mut crab8, VF, 0x01);

        Instruction::or(&mut crab8, V6, V1);

        assert_eq!(crab8.registers.get(V6), 0b00111000);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0x00);
    }

    #[test]
    fn xor() {
        let mut crab8 = Crab8::new();

        crab8.quirks.vf_reset = false;

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0b00100100);
        Instruction::store(&mut crab8, V1, 0b00111000);
        Instruction::store(&mut crab8, VF, 0x23);

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0x23);

        Instruction::xor(&mut crab8, V0, V1);

        assert_eq!(crab8.registers.get(V0), 0b00011100);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0x23);

        Instruction::store(&mut crab8, V6, 0b00000000);
        Instruction::store(&mut crab8, VF, 0x6C);

        Instruction::xor(&mut crab8, V6, V1);

        assert_eq!(crab8.registers.get(V6), 0b00111000);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0x6C);
    }

    #[test]
    fn xor_vf_reset() {
        let mut crab8 = Crab8::new();

        crab8.quirks.vf_reset = true;

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0b00100100);
        Instruction::store(&mut crab8, V1, 0b00111000);
        Instruction::store(&mut crab8, VF, 0x23);

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0x23);

        Instruction::xor(&mut crab8, V0, V1);

        assert_eq!(crab8.registers.get(V0), 0b00011100);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0x00);

        Instruction::store(&mut crab8, V6, 0b00000000);
        Instruction::store(&mut crab8, VF, 0x6C);

        Instruction::xor(&mut crab8, V6, V1);

        assert_eq!(crab8.registers.get(V6), 0b00111000);
        assert_eq!(crab8.registers.get(V1), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0x00);
    }

    #[test]
    fn shift_right() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0b00100100);
        Instruction::store(&mut crab8, V1, 0b11011011);

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b11011011);

        Instruction::shift_right(&mut crab8, V1, V0);

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b00010010);
        assert_eq!(crab8.registers.get(VF), 0x00);

        Instruction::store(&mut crab8, V2, 0b10101010);

        assert_eq!(crab8.registers.get(V2), 0b10101010);

        Instruction::shift_right(&mut crab8, V2, V1);

        assert_eq!(crab8.registers.get(V1), 0b00010010);
        assert_eq!(crab8.registers.get(V2), 0b00001001);
        assert_eq!(crab8.registers.get(VF), 0x00);

        Instruction::store(&mut crab8, V3, 0b11001100);

        assert_eq!(crab8.registers.get(V3), 0b11001100);

        Instruction::shift_right(&mut crab8, V3, V2);

        assert_eq!(crab8.registers.get(V2), 0b00001001);
        assert_eq!(crab8.registers.get(V3), 0b00000100);
        assert_eq!(crab8.registers.get(VF), 0x01);

        Instruction::store(&mut crab8, V4, 0b00011100);

        assert_eq!(crab8.registers.get(V4), 0b00011100);

        Instruction::shift_right(&mut crab8, V4, V3);

        assert_eq!(crab8.registers.get(V3), 0b00000100);
        assert_eq!(crab8.registers.get(V4), 0b00000010);
        assert_eq!(crab8.registers.get(VF), 0x00);
    }

    #[test]
    fn shift_right_quirky() {
        let mut crab8 = Crab8::new();
        crab8.quirks.shift = true;

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0b00100100);
        Instruction::store(&mut crab8, V1, 0b11011011);

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b11011011);

        Instruction::shift_right(&mut crab8, V1, V0);

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b01101101);
        assert_eq!(crab8.registers.get(VF), 0x01);

        Instruction::store(&mut crab8, V2, 0b10101010);

        assert_eq!(crab8.registers.get(V2), 0b10101010);

        Instruction::shift_right(&mut crab8, V2, V1);

        assert_eq!(crab8.registers.get(V1), 0b01101101);
        assert_eq!(crab8.registers.get(V2), 0b01010101);
        assert_eq!(crab8.registers.get(VF), 0x00);

        Instruction::store(&mut crab8, V3, 0b11001100);

        assert_eq!(crab8.registers.get(V3), 0b11001100);

        Instruction::shift_right(&mut crab8, V3, V2);

        assert_eq!(crab8.registers.get(V2), 0b01010101);
        assert_eq!(crab8.registers.get(V3), 0b01100110);
        assert_eq!(crab8.registers.get(VF), 0x00);

        Instruction::store(&mut crab8, V4, 0b00011100);

        assert_eq!(crab8.registers.get(V4), 0b00011100);

        Instruction::shift_right(&mut crab8, V4, V3);

        assert_eq!(crab8.registers.get(V3), 0b01100110);
        assert_eq!(crab8.registers.get(V4), 0b00001110);
        assert_eq!(crab8.registers.get(VF), 0x00);
    }

    #[test]
    fn shift_left() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0b00100100);
        Instruction::store(&mut crab8, V1, 0b11011011);

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b11011011);

        Instruction::shift_left(&mut crab8, V1, V0);

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b01001000);
        assert_eq!(crab8.registers.get(VF), 0x00);

        Instruction::store(&mut crab8, V2, 0b10101010);

        assert_eq!(crab8.registers.get(V2), 0b10101010);

        Instruction::shift_left(&mut crab8, V2, V1);

        assert_eq!(crab8.registers.get(V1), 0b01001000);
        assert_eq!(crab8.registers.get(V2), 0b10010000);
        assert_eq!(crab8.registers.get(VF), 0x00);

        Instruction::store(&mut crab8, V3, 0b11001100);

        assert_eq!(crab8.registers.get(V3), 0b11001100);

        Instruction::shift_left(&mut crab8, V3, V2);

        assert_eq!(crab8.registers.get(V2), 0b10010000);
        assert_eq!(crab8.registers.get(V3), 0b00100000);
        assert_eq!(crab8.registers.get(VF), 0x01);

        Instruction::store(&mut crab8, V4, 0b00011100);

        assert_eq!(crab8.registers.get(V4), 0b00011100);

        Instruction::shift_left(&mut crab8, V4, V3);

        assert_eq!(crab8.registers.get(V3), 0b00100000);
        assert_eq!(crab8.registers.get(V4), 0b01000000);
        assert_eq!(crab8.registers.get(VF), 0x00);
    }

    #[test]
    fn shift_left_quirky() {
        let mut crab8 = Crab8::new();
        crab8.quirks.shift = true;

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0b00100100);
        Instruction::store(&mut crab8, V1, 0b11011011);

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b11011011);

        Instruction::shift_left(&mut crab8, V1, V0);

        assert_eq!(crab8.registers.get(V0), 0b00100100);
        assert_eq!(crab8.registers.get(V1), 0b10110110);
        assert_eq!(crab8.registers.get(VF), 0x01);

        Instruction::store(&mut crab8, V2, 0b10101010);

        assert_eq!(crab8.registers.get(V2), 0b10101010);

        Instruction::shift_left(&mut crab8, V2, V1);

        assert_eq!(crab8.registers.get(V1), 0b10110110);
        assert_eq!(crab8.registers.get(V2), 0b01010100);
        assert_eq!(crab8.registers.get(VF), 0x01);

        Instruction::store(&mut crab8, V3, 0b11001100);

        assert_eq!(crab8.registers.get(V3), 0b11001100);

        Instruction::shift_left(&mut crab8, V3, V2);

        assert_eq!(crab8.registers.get(V2), 0b01010100);
        assert_eq!(crab8.registers.get(V3), 0b10011000);
        assert_eq!(crab8.registers.get(VF), 0x01);

        Instruction::store(&mut crab8, V4, 0b00011100);

        assert_eq!(crab8.registers.get(V4), 0b00011100);

        Instruction::shift_left(&mut crab8, V4, V3);

        assert_eq!(crab8.registers.get(V3), 0b10011000);
        assert_eq!(crab8.registers.get(V4), 0b00111000);
        assert_eq!(crab8.registers.get(VF), 0x00);
    }
}
