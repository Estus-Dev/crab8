use super::Instruction;
use crate::{memory::Address, registers::Register::*, Crab8};

impl Instruction {
    pub fn return_value(crab8: &mut Crab8) {
        let address = crab8.stack.pop().unwrap_or(Address::default());

        crab8.program_counter = address;
    }

    pub fn jump(crab8: &mut Crab8, address: Address) {
        crab8.halt_on_jump_to_self(address);
        crab8.program_counter = address;
    }

    pub fn call(crab8: &mut Crab8, address: Address) {
        crab8
            .stack
            .push(crab8.program_counter)
            .expect("Stack Overflow");

        crab8.halt_on_jump_to_self(address);
        crab8.program_counter = address;
    }

    pub fn jump_offset(crab8: &mut Crab8, address: Address) {
        let offset = crab8.registers.get(V0);
        // UNDEFINED BEHAVIOR: I'm choosing to implement overflow by wrapping.
        let address = address.wrapping_add(offset as u16);

        crab8.halt_on_jump_to_self(address);
        crab8.program_counter = address;
    }
}

impl Crab8 {
    fn halt_on_jump_to_self(&mut self, address: Address) {
        if address == self.program_counter.wrapping_sub(2) {
            self.stop();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::Instruction::*;

    #[test]
    fn jump() {
        let cases = [0x1000, 0x1234, 0x1FFF, 0x1CED, 0x12BA];

        let mut crab8 = Crab8::new();

        assert_eq!(crab8.program_counter, 0x200.into());

        for instruction in cases {
            crab8.exec(instruction);
            assert_eq!(crab8.program_counter, (instruction & 0x0FFF).into());
        }
    }

    #[test]
    fn call() {
        let cases = [0x2000, 0x2234, 0x2FFF, 0x2CED, 0x22BA];

        let mut crab8 = Crab8::new();

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
    fn jump_offset() {
        let cases = [
            (0xB000u16, 0x00u8, 0x000u16),
            (0xB123, 0x00, 0x123),
            (0xB123, 0x45, 0x168),
        ];

        let mut crab8 = Crab8::new();

        for (instruction, offset, expected) in cases {
            crab8.registers.set(V0, offset);
            crab8.exec(instruction);

            assert_eq!(crab8.program_counter, expected.into());
        }
    }
}
