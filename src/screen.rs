use itertools::Itertools;
use std::{fmt, fmt::Debug, fmt::Display, str::FromStr};
use thiserror::Error;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[derive(Clone, PartialEq, Eq)]
/// The CHIP-8 screen is a monochrome display with a width of 64px and a height of 32px.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#graphics
pub struct Screen([bool; WIDTH * HEIGHT]);

impl Screen {
    pub fn startup() -> Self {
        let screen = include_str!("../assets/sprites/crab8-startup.ch8s");
        Self::from_str(screen).unwrap()
    }

    pub fn draw(&self, x: u8, y: u8, sprite: &[u8]) -> (Self, bool) {
        let x = x as usize % WIDTH;
        let y = y as usize % HEIGHT;
        let mut screen = self.clone();
        let mut collision_flag = false;

        'y: for (sprite_y, &sprite_row) in sprite.iter().enumerate() {
            let screen_y = sprite_y + y;

            if screen_y >= HEIGHT {
                break 'y;
            }

            'x: for sprite_x in 0..8 {
                let screen_x = sprite_x + x;
                let mask = 0b_1000_0000 >> sprite_x;
                let sprite_pixel = sprite_row & mask;

                if screen_x >= WIDTH {
                    break 'x;
                }

                let i = Screen::index(screen_x, screen_y);

                if sprite_pixel > 0 {
                    let collided = self.0[i];

                    screen.0[i] = !collided;
                    collision_flag = collision_flag || collided;
                }
            }
        }

        (screen, collision_flag)
    }

    pub fn get_row(&self, y: usize) -> &[bool] {
        &self.0[Screen::index(0, y)..Screen::index(0, y + 1)]
    }

    pub fn lit(&self, x: usize, y: usize) -> bool {
        self.0[Screen::index(x, y)]
    }

    pub fn size(&self) -> (usize, usize) {
        (WIDTH, HEIGHT)
    }

    fn index(x: usize, y: usize) -> usize {
        (y * WIDTH) + x
    }
}

impl Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..HEIGHT {
            let row = self.get_row(row);
            for &pixel in row {
                write!(f, "{}", if pixel { "██" } else { "  " })?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self([false; WIDTH * HEIGHT])
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "╭{}╮", "──".repeat(WIDTH))?;

        for row in 0..HEIGHT {
            let row = self.get_row(row);
            write!(f, "│")?;

            for &pixel in row {
                write!(f, "{}", if pixel { "██" } else { "  " })?;
            }

            writeln!(f, "│")?;
        }

        writeln!(f, "╰{}╯", "──".repeat(WIDTH))?;

        Ok(())
    }
}

impl FromStr for Screen {
    type Err = ScreenParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pixels = [false; WIDTH * HEIGHT];

        for (y, line) in s.lines().enumerate() {
            if y > HEIGHT {
                return Err(ScreenParseError::InvalidHeight {
                    len: y,
                    expected: HEIGHT,
                });
            }

            for (x, pixel) in line
                .chars()
                .chunks(2)
                .into_iter()
                .enumerate()
                .map(|(column, chars)| (column, chars.collect::<String>()))
            {
                if x > WIDTH {
                    return Err(ScreenParseError::InvalidWidth {
                        line_num: y,
                        len: x,
                        expected: WIDTH * 2,
                    });
                }

                pixels[Screen::index(x, y)] = match pixel.as_str() {
                    "██" => true,
                    "  " => false,
                    _ => {
                        return Err(ScreenParseError::InvalidPixel {
                            pixel: pixel.to_owned(),
                            line_num: y,
                            column: x,
                        })
                    }
                };
            }
        }

        Ok(Screen(pixels))
    }
}

#[derive(Debug, Error)]
pub enum ScreenParseError {
    #[error("Invalid pixel {} (line {}:{}", pixel, line_num, column)]
    InvalidPixel {
        pixel: String,
        line_num: usize,
        column: usize,
    },

    #[error("Expected {} chars, found {} (line {})", len, expected, line_num)]
    InvalidWidth {
        len: usize,
        expected: usize,
        line_num: usize,
    },

    #[error("Expected {} lines, found {}", len, expected)]
    InvalidHeight { len: usize, expected: usize },
}
