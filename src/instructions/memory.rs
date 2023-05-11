use crate::{memory::Address, registers::Register, Crab8};

impl Crab8 {
    pub fn exec_store_address(&mut self, address: Address) {
        self.address_register = address;
    }

    pub fn exec_add_address(&mut self, register: Register) {
        let current_value = self.address_register;
        let value = self.registers.get(register);
        let result = current_value.wrapping_add(value as u16);

        self.address_register = result;
    }

    #[allow(clippy::identity_op)]
    pub fn exec_write_decimal(&mut self, register: Register) {
        let address = self.address_register;
        let current_value = self.registers.get(register);
        let bcd = [
            (current_value / 100) % 10,
            (current_value / 10) % 10,
            (current_value / 1) % 10,
        ];

        self.memory.set_range(address, &bcd);
    }

    pub fn exec_write(&mut self, register: Register) {
        let address = self.address_register;
        let values = self.registers.get_range(register);

        self.memory.set_range(address, values);
        self.address_register = self.address_register.wrapping_add(1 + register as u16);
    }

    pub fn exec_read(&mut self, register: Register) {
        let start = self.address_register;
        let end = start.wrapping_add(1 + register as u16);
        let values = self.memory.get_range(start, end);

        self.registers.set_range(values);
        self.address_register = self.address_register.wrapping_add(1 + register as u16);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::character::Character::*;
    use crate::instructions::Instruction::*;
    use crate::memory::FIRST_CHAR_ADDRESS;
    use crate::registers::Register::*;

    #[test]
    fn test_store_address() {
        let mut crab8 = Crab8::default();

        assert_eq!(crab8.address_register, 0x000.into());

        crab8.exec(StoreAddress(0xFFF.into()));

        assert_eq!(crab8.address_register, 0xFFF.into());

        crab8.exec(StoreAddress(0x032.into()));

        assert_eq!(crab8.address_register, 0x032.into());

        crab8.exec(StoreAddress(0x14E.into()));

        assert_eq!(crab8.address_register, 0x14E.into());
    }

    #[test]
    fn test_add_address() {
        let mut crab8 = Crab8::default();

        assert_eq!(crab8.address_register, 0x000.into());

        crab8.exec(AddAddress(V0));

        assert_eq!(crab8.address_register, 0x000.into());

        crab8.exec(Store(V0, 0x15));
        crab8.exec(AddAddress(V0));

        assert_eq!(crab8.address_register, 0x015.into());

        crab8.exec(StoreAddress(0x123.into()));

        assert_eq!(crab8.address_register, 0x123.into());

        crab8.exec(Store(V6, 0x64));
        crab8.exec(AddAddress(V6));

        assert_eq!(crab8.address_register, 0x187.into());
    }

    // This test uses bytes written in decimal for ease of use.
    #[test]
    fn test_write_decimal() {
        let mut crab8 = Crab8::default();
        let start = crab8.address_register;
        let end = start.wrapping_add(3);

        crab8.exec(Store(V8, 42));
        crab8.exec(WriteDecimal(V8));

        assert_eq!(crab8.memory.get_range(start, end), &[0, 4, 2]);

        crab8.exec(StoreAddress(0x52C.into()));

        let start = crab8.address_register;
        let end = start.wrapping_add(3);

        crab8.exec(Store(V3, 120));
        crab8.exec(WriteDecimal(V3));

        assert_eq!(crab8.memory.get_range(start, end), &[1, 2, 0]);
    }

    #[test]
    fn test_read_write() -> Result<(), ()> {
        let mut crab8 = Crab8::default();
        let mut address = Address::new(FIRST_CHAR_ADDRESS);

        crab8.address_register = address;
        crab8.exec(Read(V4));
        assert_eq!(crab8.registers.get_range(V4), Char0.sprite());
        assert_eq!(crab8.address_register, address.wrapping_add(4 + 1));

        address = Address::new(0x210);
        crab8.address_register = address;

        let result: [u8; 6] = [0x54, 0x74, 0x12, 0x62, 0xBE, 0xC0];

        for (offset, &byte) in result.iter().enumerate() {
            let register = Register::from(offset);
            crab8.exec(Store(register, byte));
        }

        crab8.exec(Write(V5));
        assert_eq!(crab8.address_register, address.wrapping_add(5 + 1));

        let start = address;
        let end = start.wrapping_add(result.len() as u16);
        assert_eq!(crab8.memory.get_range(start, end), result);

        for register in 0x0..=0xF {
            let register = Register::from(register);
            crab8.exec(Store(register, 0xBC));
        }

        crab8.exec(StoreAddress(address));
        crab8.exec(Read(V5));

        assert_eq!(crab8.registers.get_range(V5), result);

        Ok(())
    }
}
