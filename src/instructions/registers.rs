use crate::{registers::Register, Crab8};

impl Crab8 {
    pub fn exec_store(&mut self, register: Register, value: u8) {
        self.registers.set(register, value);
    }

    pub fn exec_copy(&mut self, register: Register, other: Register) {
        let value = self.registers.get(other);

        self.registers.set(register, value);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::Instruction::*;
    use crate::registers::Register::*;

    #[test]
    fn test_store() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        crab8.exec(Store(V0, 0xFF));

        assert_eq!(crab8.registers, 0xFF000000000000000000000000000000.into());

        crab8.exec(Store(V5, 0x24));

        assert_eq!(crab8.registers, 0xFF000000002400000000000000000000.into());

        crab8.exec(Store(V5, 0x00));

        assert_eq!(crab8.registers, 0xFF000000000000000000000000000000.into());
    }

    #[test]
    fn test_copy() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.registers, 0x00000000000000000000000000000000.into());

        crab8.exec(Store(V0, 0x12));

        assert_eq!(crab8.registers, 0x12000000000000000000000000000000.into());

        crab8.exec(Copy(V1, V0));

        assert_eq!(crab8.registers, 0x12120000000000000000000000000000.into());

        crab8.exec(Store(V1, 0x63));

        assert_eq!(crab8.registers, 0x12630000000000000000000000000000.into());

        crab8.exec(Copy(V8, V1));

        assert_eq!(crab8.registers, 0x12630000000000006300000000000000.into());
    }
}
