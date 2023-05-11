use crate::{memory::Address, registers::Register::*, Crab8};

impl Crab8 {
    pub fn exec_return(&mut self) {
        let address = self.stack.pop().unwrap_or(Address::default());

        self.program_counter = address;
    }

    pub fn exec_jump(&mut self, address: Address) {
        self.program_counter = address;
    }

    pub fn exec_call(&mut self, address: Address) {
        self.stack
            .push(self.program_counter)
            .expect("Stack Overflow");
        self.program_counter = address;
    }

    pub fn exec_jump_offset(&mut self, address: Address) {
        let offset = self.registers.get(V0);
        // UNDEFINED BEHAVIOR: I'm choosing to implement overflow by wrapping.
        let result = address.wrapping_add(offset as u16);

        self.program_counter = result;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::Instruction::*;

    #[test]
    fn test_jump() {
        let cases = [0x1000, 0x1234, 0x1FFF, 0x1CED, 0x12BA];

        let mut crab8 = Crab8::default();

        assert_eq!(crab8.program_counter, 0x200.into());

        for instruction in cases {
            crab8.exec(instruction);
            assert_eq!(crab8.program_counter, (instruction & 0x0FFF).into());
        }
    }

    #[test]
    fn test_call() {
        let cases = [0x2000, 0x2234, 0x2FFF, 0x2CED, 0x22BA];

        let mut crab8 = Crab8::default();

        for instruction in cases {
            crab8.exec(instruction);
            assert_eq!(crab8.program_counter, (instruction & 0x0FFF).into());
        }

        for (i, address) in cases.iter().map(|a| a & 0x0FFF).rev().skip(1).enumerate() {
            crab8.exec(Return);

            assert_eq!(crab8.program_counter, address.into(), "{i}");
        }

        crab8.exec(Return);

        assert_eq!(crab8.program_counter, Address::initial_instruction());

        crab8.exec(Return);

        assert_eq!(crab8.program_counter, Address::default());
    }

    #[test]
    fn test_jump_offset() {
        let cases = [
            (0xB000u16, 0x00u8, 0x000u16),
            (0xB123, 0x00, 0x123),
            (0xB123, 0x45, 0x168),
        ];

        let mut crab8 = Crab8::default();

        for (instruction, offset, expected) in cases {
            crab8.registers.set(V0, offset);
            crab8.exec(instruction);

            assert_eq!(crab8.program_counter, expected.into());
        }
    }
}
