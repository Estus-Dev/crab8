use super::Instruction;
use crate::{registers::Register, Crab8};

impl Instruction {
    pub fn read_delay(crab8: &mut Crab8, register: Register) {
        let result = crab8.delay.into();

        crab8.registers.set(register, result);
    }

    pub fn set_delay(crab8: &mut Crab8, register: Register) {
        let result = crab8.registers.get(register);

        crab8.delay = result.into();
    }

    pub fn set_sound(crab8: &mut Crab8, register: Register) {
        let result = crab8.registers.get(register);

        crab8.sound = result.into();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::Instruction::*;
    use crate::registers::Register::*;

    #[test]
    fn delay_timer() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.delay, 0x00.into());

        crab8.exec(Store(V5, 0x14));
        crab8.exec(SetDelay(V5));

        assert_eq!(crab8.delay, 0x14.into());

        crab8.exec(ReadDelay(V8));

        assert_eq!(crab8.registers.get(V8), 0x14);

        crab8.tick();

        assert_eq!(crab8.delay, 0x13.into());

        crab8.exec(ReadDelay(V8));

        assert_eq!(crab8.registers.get(V8), 0x13);

        crab8.exec(Store(V0, 0xFF));
        crab8.exec(SetDelay(V0));
        crab8.exec(ReadDelay(VF));

        assert_eq!(crab8.registers.get(VF), 0xFF);
    }

    #[test]
    fn sound_timer() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.sound, 0x00.into());

        crab8.exec(Store(V5, 0x14));
        crab8.exec(SetSound(V5));

        assert_eq!(crab8.sound, 0x14.into());

        crab8.tick();

        assert_eq!(crab8.sound, 0x13.into());

        crab8.exec(Store(V0, 0xFF));
        crab8.exec(SetSound(V0));
    }
}
