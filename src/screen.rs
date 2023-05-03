use std::{fmt, fmt::Debug, fmt::Display};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[derive(Clone, PartialEq, Eq)]
/// The CHIP-8 screen is a monochrome display with a width of 64px and a height of 32px.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#graphics
pub struct Screen([[bool; WIDTH]; HEIGHT]);

impl Screen {
    pub fn draw(&self, x: u8, y: u8, sprite: &[u8]) -> (Self, bool) {
        let mut screen = self.clone();
        let mut collision_flag = false;

        'y: for (sprite_y, &sprite_row) in sprite.iter().enumerate() {
            let screen_y = sprite_y + y as usize;

            if screen_y >= HEIGHT {
                break 'y;
            }

            'x: for sprite_x in 0..8 {
                let screen_x = sprite_x as usize + x as usize;
                let mask = 0b_1000_0000 >> sprite_x;
                let sprite_pixel = sprite_row & mask;

                if screen_x >= WIDTH {
                    break 'x;
                }

                let collided = self.0[screen_y][screen_x] && sprite_pixel > 0;

                screen.0[screen_y][screen_x] = sprite_pixel > 0 && !collided;
                collision_flag = collision_flag || collided;
            }
        }

        (screen, collision_flag)
    }

    pub fn get_row(&self, y: usize) -> &[bool] {
        &self.0[y]
    }

    pub fn lit(&self, x: usize, y: usize) -> bool {
        self.0[y][x]
    }

    pub fn size(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }
}

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
