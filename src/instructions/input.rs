use super::Instruction;
use crate::{input::KeyState, registers::Register, Crab8};

impl Instruction {
    pub fn if_not_pressed(crab8: &mut Crab8, register: Register) {
        let key = crab8.registers.get(register);
        let pressed = if key <= 0xF {
            crab8.input.is_key_pressed(key.into())
        } else {
            false
        };

        if pressed {
            crab8.program_counter = crab8.program_counter.next_instruction();
        }
    }

    pub fn if_pressed(crab8: &mut Crab8, register: Register) {
        let key = crab8.registers.get(register);
        let pressed = if key <= 0xF {
            crab8.input.is_key_pressed(key.into())
        } else {
            false
        };

        if !pressed {
            crab8.program_counter = crab8.program_counter.next_instruction();
        }
    }

    pub fn read_input(crab8: &mut Crab8, register: Register) {
        if let Some((key, _)) = crab8
            .input
            .into_iter()
            .find(|&(_, state)| state == KeyState::Released)
        {
            crab8.registers.set(register, key as u8);
        } else {
            crab8.program_counter = crab8.program_counter.wrapping_sub(2);
            crab8.cycle_count = crab8.cycle_count.wrapping_sub(1);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        input::{Input, Key},
        instructions::Instruction::*,
        registers::Register::{self, *},
    };

    use super::*;

    #[test]
    // TODO: Most of these tests should use some form of property-based testing
    fn is_not_pressed() {
        let mut crab8 = Crab8::new();

        for register in (0x0..=0x0F).map(Register::from) {
            for pressed_key in (0x0..=0x0F).map(Key::new) {
                crab8.input = Input::builder().set_pressed(pressed_key).build();

                for key in (0x0..=0x0F).map(Key::new) {
                    let starting_pc = crab8.program_counter;
                    let incremented_pc = crab8.program_counter.next_instruction();

                    crab8.registers.set(register, key as u8);
                    crab8.exec(IfNotPressed(register));

                    let pc = crab8.program_counter;

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
                crab8.input = Input::builder()
                    .set_pressed(pressed_key)
                    .set_pressed(second_pressed_key)
                    .build();

                for key in (0x0..=0x0F).map(Key::new) {
                    let starting_pc = crab8.program_counter;
                    let incremented_pc = crab8.program_counter.next_instruction();

                    crab8.registers.set(register, key as u8);
                    crab8.exec(IfNotPressed(register));

                    let pc = crab8.program_counter;

                    if key != pressed_key && key != second_pressed_key {
                        assert_eq!(pc, starting_pc);
                    } else {
                        assert_eq!(pc, incremented_pc);
                    }
                }
            }
        }
    }

    #[test]
    // TODO: Most of these tests should use some form of property-based testing
    fn is_pressed() {
        let mut crab8 = Crab8::new();

        for register in (0x0..=0x0F).map(Register::from) {
            for pressed_key in (0x0..=0x0F).map(Key::new) {
                crab8.input = Input::builder().set_pressed(pressed_key).build();

                for key in (0x0..=0x0F).map(Key::new) {
                    let starting_pc = crab8.program_counter;
                    let incremented_pc = crab8.program_counter.next_instruction();

                    crab8.registers.set(register, key as u8);
                    crab8.exec(IfPressed(register));

                    let pc = crab8.program_counter;

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
                crab8.input = Input::builder()
                    .set_pressed(pressed_key)
                    .set_pressed(second_pressed_key)
                    .build();

                for key in (0x0..=0x0F).map(Key::new) {
                    let starting_pc = crab8.program_counter;
                    let incremented_pc = crab8.program_counter.next_instruction();

                    crab8.registers.set(register, key as u8);
                    crab8.exec(IfPressed(register));

                    let pc = crab8.program_counter;

                    if key == pressed_key || key == second_pressed_key {
                        assert_eq!(pc, starting_pc);
                    } else {
                        assert_eq!(pc, incremented_pc);
                    }
                }
            }
        }
    }

    #[test]
    fn blocking_input() {
        let mut crab8 = Crab8::new();

        let expected = crab8.program_counter.wrapping_sub(2);
        let key = Key::KeyC;

        crab8.registers.set(V0, key.into());
        crab8.exec(ReadInput(V0));

        assert_eq!(expected, crab8.program_counter);

        crab8.program_counter = crab8.program_counter.wrapping_add(2);

        crab8.exec(ReadInput(V0));

        assert_eq!(expected, crab8.program_counter);

        crab8.input = Input::builder().set_pressed(key).build();

        crab8.program_counter = crab8.program_counter.wrapping_add(2);

        crab8.exec(ReadInput(V0));

        assert_eq!(expected, crab8.program_counter);

        crab8.input = crab8.input.update().set(key, KeyState::Pressed).build();

        crab8.program_counter = crab8.program_counter.wrapping_add(2);

        crab8.exec(ReadInput(V0));

        assert_eq!(expected, crab8.program_counter);

        crab8.input = crab8.input.update().set_released(key).build();

        crab8.program_counter = crab8.program_counter.wrapping_add(2);
        let expected = crab8.program_counter;

        crab8.exec(ReadInput(V0));

        assert_eq!(expected, crab8.program_counter);
    }
}
