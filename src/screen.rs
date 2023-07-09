use itertools::Itertools;
use std::{fmt, fmt::Debug, fmt::Display, str::FromStr};
use thiserror::Error;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

#[derive(Clone, PartialEq, Eq)]
/// The CHIP-8 screen is a monochrome display with a width of 64px and a height of 32px.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#graphics
pub struct Screen([[bool; WIDTH]; HEIGHT]);

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

                if sprite_pixel > 0 {
                    let collided = self.0[screen_y][screen_x];

                    screen.0[screen_y][screen_x] = !collided;
                    collision_flag = collision_flag || collided;
                }
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
        for row in self.0 {
            for pixel in row {
                write!(f, "{}", if pixel { "██" } else { "  " })?;
            }

            writeln!(f)?;
        }

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
        writeln!(f, "╭{}╮", "──".repeat(WIDTH))?;

        for row in self.0 {
            write!(f, "│")?;

            for pixel in row {
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
        let mut pixel_lines: Vec<[bool; WIDTH]> = Vec::with_capacity(HEIGHT);

        for (line_num, line) in s.lines().enumerate() {
            let mut pixels = Vec::with_capacity(WIDTH);

            for (column, pixel) in line
                .chars()
                .chunks(2)
                .into_iter()
                .enumerate()
                .map(|(column, chars)| (column, chars.collect::<String>()))
            {
                pixels.push(match pixel.as_str() {
                    "██" => true,
                    "  " => false,
                    _ => {
                        return Err(ScreenParseError::InvalidPixel {
                            pixel: pixel.to_owned(),
                            line_num,
                            column,
                        })
                    }
                });
            }

            if pixels.len() != WIDTH {
                return Err(ScreenParseError::InvalidWidth {
                    line_num,
                    len: line.len(),
                    expected: WIDTH * 2,
                });
            }

            if pixels.len() != WIDTH {
                return Err(ScreenParseError::InvalidWidth {
                    line_num,
                    len: line.len(),
                    expected: WIDTH * 2,
                });
            }

            let pixels = pixels[0..WIDTH].try_into().unwrap();

            pixel_lines.push(pixels);
        }

        if pixel_lines.len() != HEIGHT {
            return Err(ScreenParseError::InvalidHeight {
                len: pixel_lines.len(),
                expected: HEIGHT,
            });
        }

        let pixel_lines = pixel_lines[0..HEIGHT].try_into().unwrap();

        Ok(Screen(pixel_lines))
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
