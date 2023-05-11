use std::{fmt, fmt::Display};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// CHIP-8 timers are u8 values that tick down at a rate of 60hz.
pub struct Timer(u8);

impl Timer {
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
        write!(f, "{:#04X}", self.0)
    }
}

impl From<u8> for Timer {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<Timer> for u8 {
    fn from(value: Timer) -> Self {
        value.0
    }
}
