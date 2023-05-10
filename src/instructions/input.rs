use crate::{registers::Register, Crab8};

impl Crab8 {
    pub fn exec_if_not_pressed(&mut self, register: Register) {
        let key = self.registers.get(register);
        let pressed = if key <= 0xF {
            self.input.is_key_pressed(key.into())
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
            self.input.is_key_pressed(key.into())
        } else {
            false
        };

        if !pressed {
            self.program_counter
                .set(self.program_counter.next_instruction());
        }
    }

    pub fn exec_read_input(&mut self, register: Register) {
        self.blocking_input = Some(register);
    }
}

#[cfg(test)]
mod test {
    use crate::{
        input::{Input, Key},
        instructions::Instruction::*,
        registers::Register,
    };

    use super::*;

    #[test]
    // TODO: Most of these tests should use some form of property-based testing
    fn test_is_not_pressed() -> Result<(), ()> {
        let mut crab8 = Crab8::default();

        for register in (0x0..=0x0F).map(Register::from) {
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

        for register in (0x0..=0x0F).map(Register::from) {
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

        for register in (0x0..=0x0F).map(Register::from) {
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

        for register in (0x0..=0x0F).map(Register::from) {
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
