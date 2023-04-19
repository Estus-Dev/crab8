use std::{fmt, fmt::Debug, fmt::Display};

#[derive(Clone, PartialEq, Eq)]
/// The CHIP-8 screen is a monochrome display with a width of 64px and a height of 32px.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#graphics
pub struct Screen([[bool; 64]; 32]);

impl Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "/{}\\", "-".repeat(64))?;

        for row in self.0 {
            write!(f, "|")?;

            for pixel in row {
                write!(f, "{}", if pixel { "X" } else { " " })?;
            }

            writeln!(f, "|")?;
        }

        writeln!(f, "\\{}/", "-".repeat(64))?;

        Ok(())
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self([[false; 64]; 32])
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
