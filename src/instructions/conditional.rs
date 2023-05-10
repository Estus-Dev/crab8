use crate::{registers::Register, Crab8};

impl Crab8 {
    pub fn exec_if_not(&mut self, register: Register, value: u8) {
        let current_value = self.registers.get(register);

        if current_value == value {
            self.program_counter
                .set(self.program_counter.next_instruction());
        }
    }

    pub fn exec_if(&mut self, register: Register, value: u8) {
        let current_value = self.registers.get(register);

        if current_value != value {
            self.program_counter
                .set(self.program_counter.next_instruction());
        }
    }

    pub fn exec_if_not_registers(&mut self, register: Register, other: Register) {
        let current_value = self.registers.get(register);
        let value = self.registers.get(other);

        if current_value == value {
            self.program_counter
                .set(self.program_counter.next_instruction());
        }
    }

    pub fn exec_if_registers(&mut self, register: Register, other: Register) {
        let current_value = self.registers.get(register);
        let value = self.registers.get(other);

        if current_value != value {
            self.program_counter
                .set(self.program_counter.next_instruction());
        }
    }

    pub fn exec_if_not_pressed(&mut self, register: Register) {
        let key = self.registers.get(register);
        let pressed = if key <= 0xF {
            self.input
                .is_key_pressed(key.try_into().expect("A nibble is a valid key"))
        } else {
            false
        };

        if pressed {
            self.program_counter
                .set(self.program_counter.next_instruction());
        }
    }

    pub fn exec_if_pressed(&mut self, register: Register) {
        let key = self.registers.get(register);
        let pressed = if key <= 0xF {
            self.input
                .is_key_pressed(key.try_into().expect("A nibble is a valid key"))
        } else {
            false
        };

        if !pressed {
            self.program_counter
                .set(self.program_counter.next_instruction());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        input::{Input, Key},
        instructions::Instruction::*,
    };

    #[test]
    fn test_if_not() {
        let cases = [
            (0x3000u16, 0x00u8, true),
            (0x3000, 0x01, false),
            (0x3642, 0x42, true),
            (0x3642, 0x46, false),
        ];

        let mut crab8 = Crab8::default();

        for (instruction, value, skipped) in cases {
            let register = Register::try_from((instruction & 0x0F00) >> 8) //
                .expect("A nibble is a valid register");

            let previous_pc = crab8.program_counter.get();
            let incremented_pc = crab8.program_counter.next_instruction().get();

            crab8.exec(Store(register, value));
            crab8.exec(instruction);

            let pc = crab8.program_counter.get();

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

        let mut crab8 = Crab8::default();

        for (instruction, value, skipped) in cases {
            let register = Register::try_from((instruction & 0x0F00) >> 8) //
                .expect("A nibble is a valid register");

            let previous_pc = crab8.program_counter.get();
            let incremented_pc = crab8.program_counter.next_instruction().get();

            crab8.exec(Store(register, value));
            crab8.exec(instruction);

            let pc = crab8.program_counter.get();

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

        let mut crab8 = Crab8::default();

        for (instruction, x_value, y_value, skipped) in cases {
            let x = Register::try_from((instruction & 0x0F00) >> 8) //
                .expect("A nibble is a valid register");
            let y = Register::try_from((instruction & 0x00F0) >> 4) //
                .expect("A nibble is a valid register");

            let previous_pc = crab8.program_counter.get();
            let incremented_pc = crab8.program_counter.next_instruction().get();

            crab8.exec(Store(x, x_value));
            crab8.exec(Store(y, y_value));
            crab8.exec(instruction);

            let pc = crab8.program_counter.get();

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

        let mut crab8 = Crab8::default();

        for (instruction, x_value, y_value, skipped) in cases {
            let x = Register::try_from((instruction & 0x0F00) >> 8) //
                .expect("A nibble is a valid register");
            let y = Register::try_from((instruction & 0x00F0) >> 4) //
                .expect("A nibble is a valid register");

            let previous_pc = crab8.program_counter.get();
            let incremented_pc = crab8.program_counter.next_instruction().get();

            crab8.exec(Store(x, x_value));
            crab8.exec(Store(y, y_value));
            crab8.exec(instruction);

            let pc = crab8.program_counter.get();

            if skipped {
                assert_eq!(pc, incremented_pc);
            } else {
                assert_eq!(pc, previous_pc);
            }
        }
    }

    #[test]
    // TODO: Most of these tests should use some form of property-based testing
    fn test_is_not_pressed() -> Result<(), ()> {
        let mut crab8 = Crab8::default();

        for register in (0x0..=0x0F).map(|r| Register::try_from(r as usize).unwrap()) {
            for pressed_key in (0x0..=0x0F).map(Key::new) {
                crab8.input = Input::build().set_pressed(pressed_key, true).build();

                for key in (0x0..=0x0F).map(Key::new) {
                    let starting_pc = crab8.program_counter.get();
                    let incremented_pc = crab8.program_counter.next_instruction().get();

                    crab8.registers.set(register, key as u8);
                    crab8.exec(IfNotPressed(register));

                    let pc = crab8.program_counter.get();

                    if key != pressed_key {
                        assert_eq!(pc, starting_pc);
                    } else {
                        assert_eq!(pc, incremented_pc);
                    }
                }
            }
        }

        let second_pressed_key = Key::new(0xC);

        for register in (0x0..=0x0F).map(|r| Register::try_from(r as usize).unwrap()) {
            for pressed_key in (0x0..=0x0F).map(Key::new) {
                crab8.input = Input::build()
                    .set_pressed(pressed_key, true)
                    .set_pressed(second_pressed_key, true)
                    .build();

                for key in (0x0..=0x0F).map(Key::new) {
                    let starting_pc = crab8.program_counter.get();
                    let incremented_pc = crab8.program_counter.next_instruction().get();

                    crab8.registers.set(register, key as u8);
                    crab8.exec(IfNotPressed(register));

                    let pc = crab8.program_counter.get();

                    if key != pressed_key && key != second_pressed_key {
                        assert_eq!(pc, starting_pc);
                    } else {
                        assert_eq!(pc, incremented_pc);
                    }
                }
            }
        }

        Ok(())
    }

    #[test]
    // TODO: Most of these tests should use some form of property-based testing
    fn test_is_pressed() -> Result<(), ()> {
        let mut crab8 = Crab8::default();

        for register in (0x0..=0x0F).map(|r| Register::try_from(r as usize).unwrap()) {
            for pressed_key in (0x0..=0x0F).map(Key::new) {
                crab8.input = Input::build().set_pressed(pressed_key, true).build();

                for key in (0x0..=0x0F).map(Key::new) {
                    let starting_pc = crab8.program_counter.get();
                    let incremented_pc = crab8.program_counter.next_instruction().get();

                    crab8.registers.set(register, key as u8);
                    crab8.exec(IfPressed(register));

                    let pc = crab8.program_counter.get();

                    if key == pressed_key {
                        assert_eq!(pc, starting_pc);
                    } else {
                        assert_eq!(pc, incremented_pc);
                    }
                }
            }
        }

        let second_pressed_key = Key::new(0xC);

        for register in (0x0..=0x0F).map(|r| Register::try_from(r as usize).unwrap()) {
            for pressed_key in (0x0..=0x0F).map(Key::new) {
                crab8.input = Input::build()
                    .set_pressed(pressed_key, true)
                    .set_pressed(second_pressed_key, true)
                    .build();

                for key in (0x0..=0x0F).map(Key::new) {
                    let starting_pc = crab8.program_counter.get();
                    let incremented_pc = crab8.program_counter.next_instruction().get();

                    crab8.registers.set(register, key as u8);
                    crab8.exec(IfPressed(register));

                    let pc = crab8.program_counter.get();

                    if key == pressed_key || key == second_pressed_key {
                        assert_eq!(pc, starting_pc);
                    } else {
                        assert_eq!(pc, incremented_pc);
                    }
                }
            }
        }

        Ok(())
    }
}
