use super::Instruction;
use crate::{memory::Address, registers::Register, Crab8};

impl Instruction {
    pub fn store_address(crab8: &mut Crab8, address: Address) {
        crab8.address_register = address;
    }

    pub fn add_address(crab8: &mut Crab8, register: Register) {
        let current_value = crab8.address_register;
        let value = crab8.registers.get(register);
        let result = current_value.wrapping_add(value as u16);

        crab8.address_register = result;
    }

    #[allow(clippy::identity_op)]
    pub fn write_decimal(crab8: &mut Crab8, register: Register) {
        let address = crab8.address_register;
        let current_value = crab8.registers.get(register);
        let bcd = [
            (current_value / 100) % 10,
            (current_value / 10) % 10,
            (current_value / 1) % 10,
        ];

        crab8.memory.set_range(address, &bcd);
    }

    pub fn write(crab8: &mut Crab8, register: Register) {
        let address = crab8.address_register;
        let values = crab8.registers.get_range(register);
        let offset: u16 = (!crab8.quirks.memory_increment_by_x).into();

        crab8.memory.set_range(address, values);
        crab8.address_register = crab8
            .address_register
            .wrapping_add(offset + register as u16);
    }

    pub fn read(crab8: &mut Crab8, register: Register) {
        let start = crab8.address_register;
        let end = start.wrapping_add(1 + register as u16);
        let values = crab8.memory.get_range(start, end);
        let offset: u16 = (!crab8.quirks.memory_increment_by_x).into();

        crab8.registers.set_range(values);
        crab8.address_register = crab8
            .address_register
            .wrapping_add(offset + register as u16);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::character::Character::*;
    use crate::memory::FIRST_CHAR_ADDRESS;
    use crate::registers::Register::*;

    #[test]
    fn store_address() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.address_register, 0x000.into());

        Instruction::store_address(&mut crab8, 0xFFF.into());

        assert_eq!(crab8.address_register, 0xFFF.into());

        Instruction::store_address(&mut crab8, 0x032.into());

        assert_eq!(crab8.address_register, 0x032.into());

        Instruction::store_address(&mut crab8, 0x14E.into());

        assert_eq!(crab8.address_register, 0x14E.into());
    }

    #[test]
    fn add_address() {
        let mut crab8 = Crab8::new();

        assert_eq!(crab8.address_register, 0x000.into());

        Instruction::add_address(&mut crab8, V0);

        assert_eq!(crab8.address_register, 0x000.into());

        Instruction::store(&mut crab8, V0, 0x15);
        Instruction::add_address(&mut crab8, V0);

        assert_eq!(crab8.address_register, 0x015.into());

        Instruction::store_address(&mut crab8, 0x123.into());

        assert_eq!(crab8.address_register, 0x123.into());

        Instruction::store(&mut crab8, V6, 0x64);
        Instruction::add_address(&mut crab8, V6);

        assert_eq!(crab8.address_register, 0x187.into());
    }

    // This test uses bytes written in decimal for ease of use.
    #[test]
    fn write_decimal() {
        let mut crab8 = Crab8::new();
        let start = crab8.address_register;
        let end = start.wrapping_add(3);

        Instruction::store(&mut crab8, V8, 42);
        Instruction::write_decimal(&mut crab8, V8);

        assert_eq!(crab8.memory.get_range(start, end), &[0, 4, 2]);

        Instruction::store_address(&mut crab8, 0x52C.into());

        let start = crab8.address_register;
        let end = start.wrapping_add(3);

        Instruction::store(&mut crab8, V3, 120);
        Instruction::write_decimal(&mut crab8, V3);

        assert_eq!(crab8.memory.get_range(start, end), &[1, 2, 0]);
    }

    #[test]
    fn read_write() {
        let mut crab8 = Crab8::new();
        let mut address = Address::new(FIRST_CHAR_ADDRESS);

        crab8.address_register = address;
        Instruction::read(&mut crab8, V4);
        assert_eq!(crab8.registers.get_range(V4), Char0.sprite());
        assert_eq!(crab8.address_register, address.wrapping_add(4 + 1));

        address = Address::new(0x210);
        crab8.address_register = address;

        let result: [u8; 6] = [0x54, 0x74, 0x12, 0x62, 0xBE, 0xC0];

        for (offset, &byte) in result.iter().enumerate() {
            let register = Register::from(offset);
            Instruction::store(&mut crab8, register, byte);
        }

        Instruction::write(&mut crab8, V5);
        assert_eq!(crab8.address_register, address.wrapping_add(5 + 1));

        let start = address;
        let end = start.wrapping_add(result.len() as u16);
        assert_eq!(crab8.memory.get_range(start, end), result);

        for register in 0x0..=0xF {
            let register = Register::from(register);
            Instruction::store(&mut crab8, register, 0xBC);
        }

        Instruction::store_address(&mut crab8, address);
        Instruction::read(&mut crab8, V5);

        assert_eq!(crab8.registers.get_range(V5), result);
    }

    #[test]
    fn read_write_quirky() {
        let mut crab8 = Crab8::new();
        let mut address = Address::new(FIRST_CHAR_ADDRESS);
        crab8.quirks.memory_increment_by_x = true;

        crab8.address_register = address;
        Instruction::read(&mut crab8, V4);
        assert_eq!(crab8.registers.get_range(V4), Char0.sprite());
        assert_eq!(crab8.address_register, address.wrapping_add(4));

        address = Address::new(0x210);
        crab8.address_register = address;

        let result: [u8; 6] = [0x54, 0x74, 0x12, 0x62, 0xBE, 0xC0];

        for (offset, &byte) in result.iter().enumerate() {
            let register = Register::from(offset);
            Instruction::store(&mut crab8, register, byte);
        }

        Instruction::write(&mut crab8, V5);
        assert_eq!(crab8.address_register, address.wrapping_add(5));

        let start = address;
        let end = start.wrapping_add(result.len() as u16);
        assert_eq!(crab8.memory.get_range(start, end), result);

        for register in 0x0..=0xF {
            let register = Register::from(register);
            Instruction::store(&mut crab8, register, 0xBC);
        }

        Instruction::store_address(&mut crab8, address);
        Instruction::read(&mut crab8, V5);

        assert_eq!(crab8.registers.get_range(V5), result);
    }
}
