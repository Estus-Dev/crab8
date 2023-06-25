use crate::{registers::Register, Crab8};

impl Crab8 {
    pub fn exec_if_not(&mut self, register: Register, value: u8) {
        let current_value = self.registers.get(register);

        if current_value == value {
            self.program_counter = self.program_counter.next_instruction();
        }
    }

    pub fn exec_if(&mut self, register: Register, value: u8) {
        let current_value = self.registers.get(register);

        if current_value != value {
            self.program_counter = self.program_counter.next_instruction();
        }
    }

    pub fn exec_if_not_registers(&mut self, register: Register, other: Register) {
        let current_value = self.registers.get(register);
        let value = self.registers.get(other);

        if current_value == value {
            self.program_counter = self.program_counter.next_instruction();
        }
    }

    pub fn exec_if_registers(&mut self, register: Register, other: Register) {
        let current_value = self.registers.get(register);
        let value = self.registers.get(other);

        if current_value != value {
            self.program_counter = self.program_counter.next_instruction();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::Instruction::*;

    #[test]
    fn test_if_not() {
        let cases = [
            (0x3000u16, 0x00u8, true),
            (0x3000, 0x01, false),
            (0x3642, 0x42, true),
            (0x3642, 0x46, false),
        ];

        let mut crab8 = Crab8::new();

        for (instruction, value, skipped) in cases {
            let register = Register::from((instruction & 0x0F00) >> 8);

            let previous_pc = crab8.program_counter;
            let incremented_pc = crab8.program_counter.next_instruction();

            crab8.exec(Store(register, value));
            crab8.exec(instruction);

            let pc = crab8.program_counter;

            if skipped {
                assert_eq!(pc, incremented_pc);
            } else {
                assert_eq!(pc, previous_pc);
            }
        }
    }

    #[test]
    fn test_if() {
        let cases = [
            (0x4000u16, 0x00u8, false),
            (0x4000, 0x01, true),
            (0x4642, 0x42, false),
            (0x4642, 0x46, true),
        ];

        let mut crab8 = Crab8::new();

        for (instruction, value, skipped) in cases {
            let register = Register::from((instruction & 0x0F00) >> 8);

            let previous_pc = crab8.program_counter;
            let incremented_pc = crab8.program_counter.next_instruction();

            crab8.exec(Store(register, value));
            crab8.exec(instruction);

            let pc = crab8.program_counter;

            if skipped {
                assert_eq!(pc, incremented_pc);
            } else {
                assert_eq!(pc, previous_pc);
            }
        }
    }

    #[test]
    fn test_if_not_register() {
        let cases = [
            (0x5000u16, 0x00u8, 0x00u8, true),
            (0x5010, 0xF5, 0xF5, true),
            (0x5010, 0xF5, 0x52, false),
            (0x5640, 0x42, 0x42, true),
            (0x5640, 0x46, 0x45, false),
        ];

        let mut crab8 = Crab8::new();

        for (instruction, x_value, y_value, skipped) in cases {
            let x = Register::from((instruction & 0x0F00) >> 8);
            let y = Register::from((instruction & 0x00F0) >> 4);

            let previous_pc = crab8.program_counter;
            let incremented_pc = crab8.program_counter.next_instruction();

            crab8.exec(Store(x, x_value));
            crab8.exec(Store(y, y_value));
            crab8.exec(instruction);

            let pc = crab8.program_counter;

            if skipped {
                assert_eq!(pc, incremented_pc);
            } else {
                assert_eq!(pc, previous_pc);
            }
        }
    }

    #[test]
    fn test_if_register() {
        let cases = [
            (0x9000u16, 0x00u8, 0x00u8, false),
            (0x9010, 0xF5, 0xF5, false),
            (0x9010, 0xF5, 0x52, true),
            (0x9640, 0x42, 0x42, false),
            (0x9640, 0x46, 0x45, true),
        ];

        let mut crab8 = Crab8::new();

        for (instruction, x_value, y_value, skipped) in cases {
            let x = Register::from((instruction & 0x0F00) >> 8);
            let y = Register::from((instruction & 0x00F0) >> 4);

            let previous_pc = crab8.program_counter;
            let incremented_pc = crab8.program_counter.next_instruction();

            crab8.exec(Store(x, x_value));
            crab8.exec(Store(y, y_value));
            crab8.exec(instruction);

            let pc = crab8.program_counter;

            if skipped {
                assert_eq!(pc, incremented_pc);
            } else {
                assert_eq!(pc, previous_pc);
            }
        }
    }
}
