use super::Instruction;
use crate::{registers::Register, Crab8};

impl Instruction {
    pub fn store(crab8: &mut Crab8, register: Register, value: u8) {
        crab8.registers.set(register, value);
    }

    pub fn copy(crab8: &mut Crab8, register: Register, other: Register) {
        let value = crab8.registers.get(other);

        crab8.registers.set(register, value);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::registers::Register::*;

    #[test]
    fn store() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0xFF);

        assert_eq!(crab8.registers, 0xFF000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V5, 0x24);

        assert_eq!(crab8.registers, 0xFF000000002400000000000000000000.into());

        Instruction::store(&mut crab8, V5, 0x00);

        assert_eq!(crab8.registers, 0xFF000000000000000000000000000000.into());
    }

    #[test]
    fn copy() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        Instruction::store(&mut crab8, V0, 0x12);

        assert_eq!(crab8.registers, 0x12000000000000000000000000000000.into());

        Instruction::copy(&mut crab8, V1, V0);

        assert_eq!(crab8.registers, 0x12120000000000000000000000000000.into());

        Instruction::store(&mut crab8, V1, 0x63);

        assert_eq!(crab8.registers, 0x12630000000000000000000000000000.into());

        Instruction::copy(&mut crab8, V8, V1);

        assert_eq!(crab8.registers, 0x12630000000000006300000000000000.into());
    }
}
