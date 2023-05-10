use rand::random;

use crate::{prelude::Register, Crab8};

impl Crab8 {
    pub fn exec_rand(&mut self, register: Register, bitmask: u8) {
        let result = random::<u8>() & bitmask;

        self.registers.set(register, result);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_rand() {
        // TODO: I don't know how I want to approach testing this.
        // The bitmask needs to be tested too.
    }
}
