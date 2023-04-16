use std::{fmt, fmt::Debug, fmt::Display};

/// The CHIP-8 has 16 8-bit general-purpose registers, V0-VF.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#data-registers
#[derive(Default, PartialEq, Eq)]
pub struct Registers([u8; 16]);

impl Registers {
    /// Get the value of the selected register
    /// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#data-registers
    pub fn get(&self, register: Register) -> u8 {
        self.0[register as usize]
    }

    /// Set the value of the selected register
    /// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#data-registers
    pub fn set(&mut self, register: Register, value: u8) {
        self.0[register as usize] = value;
    }
}

impl Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = self
            .0
            .iter()
            .enumerate()
            .map(|(register, value)| {
                (
                    Register::try_from(register).expect("0x00..=0x0F are all the registers"),
                    value,
                )
            })
            .map(|(register, value)| format!("{:?}={:#04X}", register, value))
            .fold(String::new(), |str, register| str + &register + " ");

        write!(f, "{string_representation}")
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<[u8; 16]> for Registers {
    fn from(value: [u8; 16]) -> Self {
        Registers(value)
    }
}

/// General use registers on the CHIP-8 are named V0-VF.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
pub enum Register {
    /// V0 is a general use register.
    V0 = 0x00,
    /// V1 is a general use register.
    V1 = 0x01,
    /// V2 is a general use register.
    V2 = 0x02,
    /// V3 is a general use register.
    V3 = 0x03,
    /// V4 is a general use register.
    V4 = 0x04,
    /// V5 is a general use register.
    V5 = 0x05,
    /// V6 is a general use register.
    V6 = 0x06,
    /// V7 is a general use register.
    V7 = 0x07,
    /// V8 is a general use register.
    V8 = 0x08,
    /// V9 is a general use register.
    V9 = 0x09,
    /// VA is a general use register.
    VA = 0x0A,
    /// VB is a general use register.
    VB = 0x0B,
    /// VC is a general use register.
    VC = 0x0C,
    /// VD is a general use register.
    VD = 0x0D,
    /// VE is a general use register.
    VE = 0x0E,
    /// VF is a general use reigster that is often used as a flag.
    VF = 0x0F,
}

impl TryFrom<usize> for Register {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Register::try_from(value as u16)
    }
}

impl TryFrom<u16> for Register {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        use Register::*;

        match value {
            i if i == V0 as u16 => Ok(V0),
            i if i == V1 as u16 => Ok(V1),
            i if i == V2 as u16 => Ok(V2),
            i if i == V3 as u16 => Ok(V3),
            i if i == V4 as u16 => Ok(V4),
            i if i == V5 as u16 => Ok(V5),
            i if i == V6 as u16 => Ok(V6),
            i if i == V7 as u16 => Ok(V7),
            i if i == V8 as u16 => Ok(V8),
            i if i == V9 as u16 => Ok(V9),
            i if i == VA as u16 => Ok(VA),
            i if i == VB as u16 => Ok(VB),
            i if i == VC as u16 => Ok(VC),
            i if i == VD as u16 => Ok(VD),
            i if i == VE as u16 => Ok(VE),
            i if i == VF as u16 => Ok(VF),

            // TODO: Use a proper error
            _ => Err(()),
        }
    }
}
