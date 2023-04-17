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
        write!(f, "Address({:#000x?})", self.0)
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
