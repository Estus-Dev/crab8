use crate::{registers::Register, Crab8};

impl Crab8 {
    pub fn exec_read_delay(&mut self, register: Register) {
        let result = self.delay.get();

        self.registers.set(register, result);
    }

    pub fn exec_set_delay(&mut self, register: Register) {
        let result = self.registers.get(register);

        self.delay.set(result);
    }

    pub fn exec_set_sound(&mut self, register: Register) {
        let result = self.registers.get(register);

        self.sound.set(result);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::Instruction::*;
    use crate::registers::Register::*;

    #[test]
    fn test_delay_timer() {
        let mut crab8 = Crab8::default();

        assert_eq!(crab8.delay.get(), 0x00);

        crab8.exec(Store(V5, 0x14));
        crab8.exec(SetDelay(V5));

        assert_eq!(crab8.delay.get(), 0x14);

        crab8.exec(ReadDelay(V8));

        assert_eq!(crab8.registers.get(V8), 0x14);

        crab8.tick();

        assert_eq!(crab8.delay.get(), 0x13);

        crab8.exec(ReadDelay(V8));

        assert_eq!(crab8.registers.get(V8), 0x13);

        crab8.exec(Store(V0, 0xFF));
        crab8.exec(SetDelay(V0));
        crab8.exec(ReadDelay(VF));

        assert_eq!(crab8.registers.get(VF), 0xFF);
    }

    #[test]
    fn test_sound_timer() {
        let mut crab8 = Crab8::default();

        assert_eq!(crab8.sound.get(), 0x00);

        crab8.exec(Store(V5, 0x14));
        crab8.exec(SetSound(V5));

        assert_eq!(crab8.sound.get(), 0x14);

        crab8.tick();

        assert_eq!(crab8.sound.get(), 0x13);

        crab8.exec(Store(V0, 0xFF));
        crab8.exec(SetSound(V0));
    }
}
