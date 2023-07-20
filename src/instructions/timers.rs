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
    use crate::registers::Register::*;

    #[test]
    fn delay_timer() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.delay, 0x00.into());

        Instruction::store(&mut crab8, V5, 0x14);
        Instruction::set_delay(&mut crab8, V5);

        assert_eq!(crab8.delay, 0x14.into());

        Instruction::read_delay(&mut crab8, V8);

        assert_eq!(crab8.registers.get(V8), 0x14);

        crab8.tick();

        assert_eq!(crab8.delay, 0x13.into());

        Instruction::read_delay(&mut crab8, V8);

        assert_eq!(crab8.registers.get(V8), 0x13);

        Instruction::store(&mut crab8, V0, 0xFF);
        Instruction::set_delay(&mut crab8, V0);
        Instruction::read_delay(&mut crab8, VF);

        assert_eq!(crab8.registers.get(VF), 0xFF);
    }

    #[test]
    fn sound_timer() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.sound, 0x00.into());

        Instruction::store(&mut crab8, V5, 0x14);
        Instruction::set_sound(&mut crab8, V5);

        assert_eq!(crab8.sound, 0x14.into());

        crab8.tick();

        assert_eq!(crab8.sound, 0x13.into());

        Instruction::store(&mut crab8, V0, 0xFF);
        Instruction::set_sound(&mut crab8, V0);
    }
}
