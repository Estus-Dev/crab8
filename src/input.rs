use crate::prelude::*;
use std::{fmt, fmt::Debug, fmt::Display};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Key0 = 0x0,
    Key1 = 0x1,
    Key2 = 0x2,
    Key3 = 0x3,
    Key4 = 0x4,
    Key5 = 0x5,
    Key6 = 0x6,
    Key7 = 0x7,
    Key8 = 0x8,
    Key9 = 0x9,
    KeyA = 0xA,
    KeyB = 0xB,
    KeyC = 0xC,
    KeyD = 0xD,
    KeyE = 0xE,
    KeyF = 0xF,
}

impl Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key = format!("{:#X}", *self as usize).replace("0x", "");

        write!(f, "Key({key})")
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let key = format!("{:#X}", *self as usize).replace("0x", "");

        write!(f, "{key}")
    }
}

impl TryFrom<u8> for Key {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Key0),
            0x1 => Ok(Key1),
            0x2 => Ok(Key2),
            0x3 => Ok(Key3),
            0x4 => Ok(Key4),
            0x5 => Ok(Key5),
            0x6 => Ok(Key6),
            0x7 => Ok(Key7),
            0x8 => Ok(Key8),
            0x9 => Ok(Key9),
            0xA => Ok(KeyA),
            0xB => Ok(KeyB),
            0xC => Ok(KeyC),
            0xD => Ok(KeyD),
            0xE => Ok(KeyE),
            0xF => Ok(KeyF),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Input([bool; 16]);

impl Input {
    pub fn build() -> InputBuilder {
        InputBuilder::default()
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.0[key as usize]
    }
}

impl Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pressed_keys = (0x0..=0xF)
            .enumerate()
            .map(|(i, key)| (i, self.0[key]))
            .filter_map(|(i, key)| if key { Some(i) } else { None })
            .map(|key| format!("{key:#3X}").replace("0x", ""))
            .fold("".to_owned(), |pressed_keys, key| pressed_keys + " " + &key)
            .trim()
            .to_owned();

        write!(f, "{pressed_keys}")
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl IntoIterator for Input {
    type Item = (Key, bool);
    type IntoIter = InputIterator;

    fn into_iter(self) -> Self::IntoIter {
        InputIterator(self, 0x0)
    }
}

#[derive(Default)]
pub struct InputBuilder([bool; 16]);

impl InputBuilder {
    pub fn set_pressed(mut self, key: Key, pressed: bool) -> Self {
        self.0[key as usize] = pressed;

        self
    }

    pub fn build(self) -> Input {
        Input(self.0)
    }
}

pub struct InputIterator(Input, usize);

impl Iterator for InputIterator {
    type Item = (Key, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let next_index = 1 + self.1 as usize;

        if next_index <= 0xF {
            Some((
                Key::try_from(1 + next_index as u8).unwrap(),
                self.0 .0[next_index + 1],
            ))
        } else {
            None
        }
    }
}
