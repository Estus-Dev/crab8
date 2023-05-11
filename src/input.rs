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

impl Key {
    pub fn new(key: u8) -> Self {
        key.into()
    }
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

impl From<u8> for Key {
    fn from(value: u8) -> Self {
        match value & 0x0F {
            0x0 => Key0,
            0x1 => Key1,
            0x2 => Key2,
            0x3 => Key3,
            0x4 => Key4,
            0x5 => Key5,
            0x6 => Key6,
            0x7 => Key7,
            0x8 => Key8,
            0x9 => Key9,
            0xA => KeyA,
            0xB => KeyB,
            0xC => KeyC,
            0xD => KeyD,
            0xE => KeyE,
            0xF => KeyF,
            _ => unreachable!("Always converts the last nibble to a key"),
        }
    }
}

impl From<Key> for u8 {
    fn from(value: Key) -> Self {
        value as u8
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KeyState {
    #[default]
    Unpressed,
    Pressed,
    Released,
}

#[derive(Clone, Copy, Default)]
pub struct Input([KeyState; 16]);

impl Input {
    pub fn builder() -> InputBuilder {
        InputBuilder::new()
    }

    pub fn update(&self) -> InputBuilder {
        InputBuilder::new_from_previous(self)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.0[key as usize] == KeyState::Pressed
    }

    pub fn was_key_released(&self, key: Key) -> bool {
        self.0[key as usize] == KeyState::Unpressed
    }
}

impl Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pressed_keys = (0x0..=0xF)
            .enumerate()
            .map(|(i, key)| (i, self.0[key]))
            .filter_map(|(i, key)| {
                if key == KeyState::Pressed {
                    Some(i)
                } else {
                    None
                }
            })
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
    type Item = (Key, KeyState);
    type IntoIter = InputIterator;

    fn into_iter(self) -> Self::IntoIter {
        InputIterator(self, 0x0)
    }
}

#[derive(Debug, Default)]
pub struct InputBuilder([KeyState; 16]);

impl InputBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_from_previous(previous: &Input) -> Self {
        use KeyState::*;

        Self(previous.0.map(|key| match key {
            Released => Unpressed,
            _ => key,
        }))
    }

    pub fn set_pressed(&mut self, key: Key) -> &mut Self {
        self.set(key, KeyState::Pressed)
    }

    pub fn set_released(&mut self, key: Key) -> &mut Self {
        if self.0[key as usize] != KeyState::Pressed {
            println!("Released {key} when it was not pressed");
        }

        self.set(key, KeyState::Released)
    }

    pub fn set(&mut self, key: Key, state: KeyState) -> &mut Self {
        self.0[key as usize] = state;

        self
    }

    pub fn build(&self) -> Input {
        Input(self.0)
    }
}

pub struct InputIterator(Input, usize);

impl Iterator for InputIterator {
    type Item = (Key, KeyState);

    fn next(&mut self) -> Option<Self::Item> {
        self.1 += 1;

        if self.1 <= 0xF {
            Some((Key::new(self.1 as u8), self.0 .0[self.1]))
        } else {
            None
        }
    }
}
