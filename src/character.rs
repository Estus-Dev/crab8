use crate::memory::{CHAR_SPRITE_WIDTH, FIRST_CHAR_ADDRESS};
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]

pub enum Character {
    Char0 = 0x0,
    Char1 = 0x1,
    Char2 = 0x2,
    Char3 = 0x3,
    Char4 = 0x4,
    Char5 = 0x5,
    Char6 = 0x6,
    Char7 = 0x7,
    Char8 = 0x8,
    Char9 = 0x9,
    CharA = 0xA,
    CharB = 0xB,
    CharC = 0xC,
    CharD = 0xD,
    CharE = 0xE,
    CharF = 0xF,
}

impl Character {
    pub fn address(&self) -> Address {
        let first = Address::try_from(FIRST_CHAR_ADDRESS).unwrap();
        let offset = *self as u16 * CHAR_SPRITE_WIDTH;

        first.wrapping_add(offset)
    }

    pub fn sprite(&self) -> &[u8] {
        match self {
            Char0 => &[
                0b_1111_0000, // XXXX
                0b_1001_0000, // X  X
                0b_1001_0000, // X  X
                0b_1001_0000, // X  X
                0b_1111_0000, // XXXX
            ],
            Char1 => &[
                0b_0010_0000, //   X
                0b_0110_0000, //  XX
                0b_0010_0000, //   X
                0b_0010_0000, //   X
                0b_0111_0000, //  XXX
            ],
            Char2 => &[
                0b_1111_0000, // XXXX
                0b_0001_0000, //    X
                0b_1111_0000, // XXXX
                0b_1000_0000, // X
                0b_1111_0000, // XXXX
            ],
            Char3 => &[
                0b_1111_0000, // XXXX
                0b_0001_0000, //    X
                0b_1111_0000, // XXXX
                0b_0001_0000, //    X
                0b_1111_0000, // XXXX
            ],
            Char4 => &[
                0b_1001_0000, // X  X
                0b_1001_0000, // X  X
                0b_1111_0000, // XXXX
                0b_0001_0000, //    X
                0b_0001_0000, //    X
            ],
            Char5 => &[
                0b_1111_0000, // XXXX
                0b_1000_0000, // X
                0b_1111_0000, // XXXX
                0b_0001_0000, //    X
                0b_1111_0000, // XXXX
            ],
            Char6 => &[
                0b_1111_0000, // XXXX
                0b_1000_0000, // X
                0b_1111_0000, // XXXX
                0b_1001_0000, // X  X
                0b_1111_0000, // XXXX
            ],
            Char7 => &[
                0b_1111_0000, // XXXX
                0b_0001_0000, //    X
                0b_0010_0000, //   X
                0b_0100_0000, //  X
                0b_0100_0000, //  X
            ],
            Char8 => &[
                0b_1111_0000, // XXXX
                0b_1001_0000, // X  X
                0b_1111_0000, // XXXX
                0b_1001_0000, // X  X
                0b_1111_0000, // XXXX
            ],
            Char9 => &[
                0b_1111_0000, // XXXX
                0b_1001_0000, // X  X
                0b_1111_0000, // XXXX
                0b_0001_0000, //    X
                0b_1111_0000, // XXXX
            ],
            CharA => &[
                0b_1111_0000, // XXXX
                0b_1001_0000, // X  X
                0b_1111_0000, // XXXX
                0b_1001_0000, // X  X
                0b_1001_0000, // X  X
            ],
            CharB => &[
                0b_1110_0000, // XXX
                0b_1001_0000, // X  X
                0b_1110_0000, // XXX
                0b_1001_0000, // X  X
                0b_1110_0000, // XXX
            ],
            CharC => &[
                0b_1111_0000, // XXXX
                0b_1000_0000, // X
                0b_1000_0000, // X
                0b_1000_0000, // X
                0b_1111_0000, // XXXX
            ],
            CharD => &[
                0b_1110_0000, // XXX
                0b_1001_0000, // X  X
                0b_1001_0000, // X  X
                0b_1001_0000, // X  X
                0b_1110_0000, // XXX
            ],
            CharE => &[
                0b_1111_0000, // XXXX
                0b_1000_0000, // X
                0b_1111_0000, // XXXX
                0b_1000_0000, // X
                0b_1111_0000, // XXXX
            ],
            CharF => &[
                0b_1111_0000, // XXXX
                0b_1000_0000, // X
                0b_1111_0000, // XXXX
                0b_1000_0000, // X
                0b_1000_0000, // X
            ],
        }
    }
}

impl From<u8> for Character {
    /// Convert the last nibble of a byte into a character. Lossy for values > 0xF.
    fn from(value: u8) -> Self {
        match value & 0x0F {
            0x0 => Char0,
            0x1 => Char1,
            0x2 => Char2,
            0x3 => Char3,
            0x4 => Char4,
            0x5 => Char5,
            0x6 => Char6,
            0x7 => Char7,
            0x8 => Char8,
            0x9 => Char9,
            0xA => CharA,
            0xB => CharB,
            0xC => CharC,
            0xD => CharD,
            0xE => CharE,
            0xF => CharF,

            _ => unreachable!("Always converts the last nibble to a character"),
        }
    }
}
