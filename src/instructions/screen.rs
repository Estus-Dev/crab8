use super::Instruction;
use crate::{
    character::Character,
    memory::{Address, CHAR_SPRITE_WIDTH, FIRST_CHAR_ADDRESS},
    registers::{Register, Register::*},
    screen::Screen,
    Crab8,
};

impl Instruction {
    pub fn clear_screen(crab8: &mut Crab8) {
        crab8.screen = Screen::default();
    }

    pub fn draw(crab8: &mut Crab8, x: Register, y: Register, row_count: u8) {
        if crab8.quirks.display_wait && crab8.instructions_since_frame > 0 {
            crab8.program_counter = crab8.program_counter.wrapping_sub(2);
            crab8.cycle_count = crab8.cycle_count.wrapping_sub(1);
            return;
        }

        let start = crab8.address_register;
        let end = start.wrapping_add(row_count as u16);
        let x = crab8.registers.get(x);
        let y = crab8.registers.get(y);
        let sprite = crab8.memory.get_range(start, end);

        let (screen, collision_flag) = crab8.screen.draw(x, y, sprite);

        crab8.screen = screen;
        crab8.registers.set(VF, collision_flag as u8);
    }

    pub fn load_sprite(crab8: &mut Crab8, register: Register) {
        let first_address = Address::new(FIRST_CHAR_ADDRESS);
        let current_value = crab8.registers.get(register);

        // Converting to character here will wrap out of bounds values
        let character: Character = current_value.into();

        let offset = CHAR_SPRITE_WIDTH * character as u16;
        let result = first_address.wrapping_add(offset);

        crab8.address_register = result;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_sprite() {
        let mut crab8 = Crab8::new();
        let mut offset = 0x00;

        assert_eq!(crab8.address_register, 0x000.into());

        Instruction::store(&mut crab8, V5, 0x00);
        Instruction::load_sprite(&mut crab8, V5);

        assert_eq!(crab8.address_register, (FIRST_CHAR_ADDRESS + offset).into());

        Instruction::store(&mut crab8, V3, 0x04);
        Instruction::load_sprite(&mut crab8, V3);

        offset = 0x04 * CHAR_SPRITE_WIDTH;
        assert_eq!(crab8.address_register, (FIRST_CHAR_ADDRESS + offset).into());

        Instruction::store(&mut crab8, VB, 0x0F);
        Instruction::load_sprite(&mut crab8, VB);

        offset = 0x0F * CHAR_SPRITE_WIDTH;
        assert_eq!(crab8.address_register, (FIRST_CHAR_ADDRESS + offset).into());
    }
}
