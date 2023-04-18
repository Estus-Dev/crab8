use std::{fmt, fmt::Display};

#[derive(Debug, Default)]
/// CHIP-8 timers are u8 values that tick down at a rate of 60hz.
pub struct Timer(u8);

impl Timer {
    pub fn get(&self) -> u8 {
        self.0
    }

    pub fn set(&mut self, value: u8) {
        self.0 = value;
    }

    pub fn tick(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    pub fn is_active(&self) -> bool {
        self.0 > 0
    }
}

impl Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
