use super::Instruction;
use crate::{prelude::Register, Crab8};
use rand::random;

impl Instruction {
    pub fn rand(crab8: &mut Crab8, register: Register, bitmask: u8) {
        let result = random::<u8>() & bitmask;

        crab8.registers.set(register, result);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn rand() {
        // TODO: I don't know how I want to approach testing this.
        // The bitmask needs to be tested too.
    }
}
