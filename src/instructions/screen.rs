use crate::{
    character::Character,
    memory::{Address, CHAR_SPRITE_WIDTH, FIRST_CHAR_ADDRESS},
    registers::{Register, Register::*},
    screen::Screen,
    Crab8,
};

impl Crab8 {
    pub fn exec_clear_screen(&mut self) {
        self.screen = Screen::default();
    }

    pub fn exec_draw(&mut self, x: Register, y: Register, row_count: u8) {
        let start = self.address_register;
        let end = start.wrapping_add(row_count as u16);
        let x = self.registers.get(x);
        let y = self.registers.get(y);
        let sprite = self.memory.get_range(start, end);

        let (screen, collision_flag) = self.screen.draw(x, y, sprite);

        self.screen = screen;
        self.registers.set(VF, collision_flag as u8);
    }

    pub fn exec_load_sprite(&mut self, register: Register) {
        let first_address = Address::new(FIRST_CHAR_ADDRESS);
        let current_value = self.registers.get(register);

        // Converting to character here will wrap out of bounds values
        let character: Character = current_value.into();

        let offset = CHAR_SPRITE_WIDTH * character as u16;
        let result = first_address.wrapping_add(offset);

        self.address_register = result;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::Instruction::*;

    #[test]
    fn test_clear_screen() {
        let mut crab8 = Crab8::default();

        // TODO: Draw something

        crab8.exec(ClearScreen);

        assert_eq!(crab8.screen, Screen::default());
    }

    #[test]
    fn test_draw() {
        // TODO:
    }

    #[test]
    fn test_load_sprite() {
        let mut crab8 = Crab8::default();
        let mut offset = 0x00;

        assert_eq!(crab8.address_register.get(), 0x000);

        crab8.exec(Store(V5, 0x00));
        crab8.exec(LoadSprite(V5));

        assert_eq!(crab8.address_register.get(), FIRST_CHAR_ADDRESS + offset);

        crab8.exec(Store(V3, 0x04));
        crab8.exec(LoadSprite(V3));

        offset = 0x04 * CHAR_SPRITE_WIDTH;
        assert_eq!(crab8.address_register.get(), FIRST_CHAR_ADDRESS + offset);

        crab8.exec(Store(VB, 0x0F));
        crab8.exec(LoadSprite(VB));

        offset = 0x0F * CHAR_SPRITE_WIDTH;
        assert_eq!(crab8.address_register.get(), FIRST_CHAR_ADDRESS + offset);
    }
}
