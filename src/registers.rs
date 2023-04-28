#[cfg(feature = "bevy")]
use bevy::ecs::component::Component;
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

    pub fn get_range(&self, register: Register) -> &[u8] {
        &self.0[0x0..=register as usize]
    }

    /// Set the value of the selected register
    /// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#data-registers
    pub fn set(&mut self, register: Register, value: u8) {
        self.0[register as usize] = value;
    }

    pub fn set_range(&mut self, values: &[u8]) {
        for (offset, &value) in values.iter().enumerate() {
            self.0[offset] = value;
        }
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
        Self(value)
    }
}

#[cfg(test)]
impl From<u128> for Registers {
    #[allow(clippy::identity_op, clippy::erasing_op)]
    /// Convert from hex octets to Registers from left to right.
    /// Used for quick test literals.
    fn from(value: u128) -> Self {
        Self([
            ((value & 0xFF000000000000000000000000000000) >> (8 * 0xF)) as u8,
            ((value & 0x00FF0000000000000000000000000000) >> (8 * 0xE)) as u8,
            ((value & 0x0000FF00000000000000000000000000) >> (8 * 0xD)) as u8,
            ((value & 0x000000FF000000000000000000000000) >> (8 * 0xC)) as u8,
            ((value & 0x00000000FF0000000000000000000000) >> (8 * 0xB)) as u8,
            ((value & 0x0000000000FF00000000000000000000) >> (8 * 0xA)) as u8,
            ((value & 0x000000000000FF000000000000000000) >> (8 * 0x9)) as u8,
            ((value & 0x00000000000000FF0000000000000000) >> (8 * 0x8)) as u8,
            ((value & 0x0000000000000000FF00000000000000) >> (8 * 0x7)) as u8,
            ((value & 0x000000000000000000FF000000000000) >> (8 * 0x6)) as u8,
            ((value & 0x00000000000000000000FF0000000000) >> (8 * 0x5)) as u8,
            ((value & 0x0000000000000000000000FF00000000) >> (8 * 0x4)) as u8,
            ((value & 0x000000000000000000000000FF000000) >> (8 * 0x3)) as u8,
            ((value & 0x00000000000000000000000000FF0000) >> (8 * 0x2)) as u8,
            ((value & 0x0000000000000000000000000000FF00) >> (8 * 0x1)) as u8,
            ((value & 0x000000000000000000000000000000FF) >> (8 * 0x0)) as u8,
        ])
    }
}

/// General use registers on the CHIP-8 are named V0-VF.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "bevy", derive(Component))]
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

impl Register {
    pub fn name(&self) -> &str {
        match self {
            Self::V0 => "V0",
            Self::V1 => "V1",
            Self::V2 => "V2",
            Self::V3 => "V3",
            Self::V4 => "V4",
            Self::V5 => "V5",
            Self::V6 => "V6",
            Self::V7 => "V7",
            Self::V8 => "V8",
            Self::V9 => "V9",
            Self::VA => "VA",
            Self::VB => "VB",
            Self::VC => "VC",
            Self::VD => "VD",
            Self::VE => "VE",
            Self::VF => "VF",
        }
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_registers_from_u128() {
        let cases: [(u128, [u8; 16]); 2] = [
            (
                0x000000000000000000000000000000FF,
                [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF,
                ],
            ),
            (
                0x0123456789ABCDEF0123456789ABCDEF,
                [
                    0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, //
                    0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
                ],
            ),
        ];

        for case in cases {
            assert_eq!(Registers::from(case.0), Registers::from(case.1));
        }
    }
}
