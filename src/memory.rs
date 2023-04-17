/// The CHIP-8 has 12-bit addresses, allowing up to 4096 bytes of memory.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#storage-in-memory
#[derive(Debug, Default, PartialEq, Eq)]
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
