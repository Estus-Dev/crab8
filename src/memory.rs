use std::{fmt, fmt::Debug, fmt::Display, fmt::Formatter};

/// The CHIP-8 has 12-bit addresses, allowing up to 4096 bytes of memory.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#storage-in-memory
#[derive(Default, PartialEq, Eq)]
pub struct Address(u16);

impl Address {
    /// CHIP-8 programs are loaded starting at 0x200.
    /// Values below this are reserved for the interpreter.
    pub fn starting_address() -> Address {
        Self(0x200)
    }

    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn set(&mut self, address: Address) {
        self.0 = address.get();
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Address({:#05x?})", self.0)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl TryFrom<u16> for Address {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < 0xF000 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

pub struct Memory([u8; 4096]);

impl Debug for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        const CHUNK_SIZE: usize = 16;

        writeln!(f, "       00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F")?;

        for (i, row) in self.0.chunks(CHUNK_SIZE).enumerate() {
            let bytes_string = row
                .iter()
                .map(|&b| format!("{b:#04X?}").replace("0x", ""))
                .fold("".to_owned(), |bytes_string, s| bytes_string + " " + &s)
                .trim()
                .to_owned();

            writeln!(f, "{:#05X}: {bytes_string}", i * CHUNK_SIZE)?
        }

        Ok(())
    }
}

impl Default for Memory {
    fn default() -> Self {
        let mut default = Self([0x00; 4096]);

        // Fill reserved address space with 0xFF for visualization purposes.

        for address in 0x000..0x200 {
            default.0[address] = 0xFF;
        }

        for address in 0xE90..=0xFFF {
            default.0[address] = 0xFF;
        }

        default
    }
}

impl Display for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
