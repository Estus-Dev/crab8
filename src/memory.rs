use crate::prelude::*;
use std::{fmt, fmt::Debug, fmt::Display, fmt::Formatter};

/// The first safe memory address is 0x200.
/// Values below this address are reserved for the CHIP-8 interpreter.
const FIRST_SAFE_ADDRESS: u16 = 0x200;

/// The program counter is initialized to 0x200 to start.
const INITIAL_PC: u16 = FIRST_SAFE_ADDRESS;

/// The last memory address is 0xFFF, giving 4096 bytes of memory total.
const LAST_ADDRESS: u16 = 0xFFF;

/// The last 352 bytes are reserved for "variables and display refresh".
const LAST_SAFE_ADDRESS: u16 = LAST_ADDRESS - 352;

// Character sprites are 5 bytes wide.
pub const CHAR_SPRITE_WIDTH: u16 = 5;

/// The beginning of the reserved addresses will be used for sprite data.
pub const FIRST_CHAR_ADDRESS: u16 = 0x000;

/// The CHIP-8 has 12-bit addresses, allowing up to 4096 bytes of memory.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#storage-in-memory
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct Address(u16);

impl Address {
    /// Create a new address. The top nibble will be discarded.
    pub fn new(address: u16) -> Self {
        address.into()
    }

    /// CHIP-8 programs are loaded starting at 0x200.
    /// Values below this are reserved for the interpreter.
    pub fn initial_instruction() -> Self {
        Self(INITIAL_PC)
    }

    /// Add a u16 to the given Address and return a new Address.
    pub fn wrapping_add(&self, value: u16) -> Address {
        Address((self.0 + value) & 0x0FFF)
    }

    /// Subtract a u16 from the given Address and return a new Address.
    pub fn wrapping_sub(&self, value: u16) -> Address {
        Address(self.0.wrapping_sub(value) & 0x0FFF)
    }

    /// Get the address of the next byte in memory
    pub fn next(&self) -> Address {
        self.wrapping_add(1)
    }

    /// Get the address of the next instruction in memory
    pub fn next_instruction(&self) -> Address {
        self.wrapping_add(2)
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<u16> for Address {
    fn from(value: u16) -> Self {
        Self(value & 0x0FFF)
    }
}

impl From<Address> for u16 {
    fn from(value: Address) -> Self {
        value.0
    }
}

impl From<Address> for usize {
    fn from(value: Address) -> Self {
        value.0 as usize
    }
}

impl std::fmt::UpperHex for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::UpperHex::fmt(&self.0, f)
    }
}

pub struct Memory([u8; 4096]);

impl Memory {
    /// Get the value of an Address in memory.
    pub fn get(&self, address: Address) -> u8 {
        // The safety of this relies on not being able to construct an invalid Address.
        // This also assumed 4096 sized memory. For 2048 sized memory that needs a smaller Address.
        self.0[address.0 as usize]
    }

    pub fn get_instruction(&self, address: Address) -> Instruction {
        let next_address = address.next().0 as usize;
        let address = address.0 as usize;
        let instruction = ((self.0[address] as u16) << 8) + self.0[next_address] as u16;

        Instruction::from(instruction)
    }

    pub fn get_range(&self, start: Address, end: Address) -> &[u8] {
        let start = start.into();
        let mut end = end.into();

        if start >= end {
            end = start;
        }

        &self.0[start..end]
    }

    /// Set the value of an address in memory.
    pub fn set(&mut self, address: Address, value: u8) {
        self.0[address.0 as usize] = value;
    }

    pub fn set_range(&mut self, address: Address, values: &[u8]) {
        for (offset, &value) in values.iter().enumerate() {
            self.set(address.wrapping_add(offset as u16), value);
        }
    }

    // TODO: Take an Instruction instead
    pub fn set_instruction(&mut self, address: Address, instruction: u16) {
        let instruction = [
            ((instruction & 0xFF00) >> 8) as u8,
            (instruction & 0x00FF) as u8,
        ];

        self.set_range(address, &instruction);
    }

    /// An iterator over each address and the byte in memory at that address
    pub fn iter(&self) -> MemoryIter {
        MemoryIter {
            memory: self,
            address: Address::new(0x000),
        }
    }

    /// An iterator over every other address and the instruction at that address
    pub fn iter_instructions(&self) -> InstructionIter {
        InstructionIter {
            memory: self,
            address: Address::new(0x000),
        }
    }
}

impl Debug for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        const CHUNK_SIZE: usize = 16;

        writeln!(f, "       00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F")?;

        for (i, row) in self.0.chunks(CHUNK_SIZE).enumerate() {
            let row_address = CHUNK_SIZE * i;
            let bytes_string = row
                .iter()
                .map(|&b| format!("{b:#04X?}").replace("0x", ""))
                .fold("".to_owned(), |bytes_string, s| bytes_string + " " + &s)
                .trim()
                .to_owned();

            writeln!(f, "{row_address:#05X}: {bytes_string}")?
        }

        Ok(())
    }
}

impl Default for Memory {
    fn default() -> Self {
        let mut default = Self([0x00; 4096]);
        let char_sprite_end = FIRST_CHAR_ADDRESS + (16 * CHAR_SPRITE_WIDTH);

        // Fill in sprite data
        for (char, address) in (FIRST_CHAR_ADDRESS..char_sprite_end)
            .step_by(CHAR_SPRITE_WIDTH as usize)
            .enumerate()
        {
            let char: Character = (char as u8).into();
            default.set_range(address.into(), char.sprite());
        }

        // Fill starting reserved address space with 0xFF for visualization purposes.
        for address in char_sprite_end..FIRST_SAFE_ADDRESS {
            default.0[address as usize] = 0xFF;
        }

        // At the end of valid address space, jump back to 0x200
        default.set_instruction(Address::new(LAST_SAFE_ADDRESS + 1), 0x1200);
        // TODO: default.set_instruction(Address::new(LAST_SAFE_ADDRESS + 1), Jump(0x200));

        // Fill ending reserved address space with 0xFF for visualization purposes.
        for address in (LAST_SAFE_ADDRESS + 3)..=LAST_ADDRESS {
            default.0[address as usize] = 0xFF;
        }

        default
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

pub struct MemoryIter<'a> {
    memory: &'a Memory,
    address: Address,
}

impl<'a> Iterator for MemoryIter<'a> {
    type Item = (Address, u8);

    fn next(&mut self) -> Option<Self::Item> {
        self.nth(self.address.0 as usize)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        // Here we go all the way to the last address
        let max = self.memory.0.len();

        if n < max {
            let addr = Address::new(n as u16);
            self.address.0 = n as u16 + 1;

            Some((addr, self.memory.get(addr)))
        } else {
            None
        }
    }
}

pub struct InstructionIter<'a> {
    memory: &'a Memory,
    address: Address,
}

impl<'a> Iterator for InstructionIter<'a> {
    type Item = (Address, Instruction);

    fn next(&mut self) -> Option<Self::Item> {
        self.nth(self.address.0 as usize)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        // Here we go to the address before last. Instructions are 16 bits wide.
        let max = self.memory.0.len() - 1;

        if n < max {
            let addr = Address::new(n as u16);
            self.address.0 = n as u16 + 2;

            Some((addr, self.memory.get_instruction(addr)))
        } else {
            None
        }
    }
}
