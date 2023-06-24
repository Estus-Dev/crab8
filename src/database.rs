#![allow(dead_code)] // TODO:

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Program {
    title: String,
    description: Option<String>,
    // TODO: This should be a date of some kind
    release: Option<String>,
    origin: Option<Origin>,
    copyright: Option<String>,
    // TODO: See https://crates.io/crates/spdx
    license: Option<String>,
    authors: Option<Vec<String>>,
    // TODO: Support real images here
    images: Option<Vec<String>>,
    // TODO: Use an appropriate URL type here
    urls: Option<Vec<String>>,
    // TODO: Use an appropriate type for hashes
    roms: HashMap<String, Rom>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum OriginType {
    #[serde(rename = "gamejam")]
    GameJam,

    #[serde(rename = "event")]
    Event,

    #[serde(rename = "magazine")]
    Magazine,

    #[serde(rename = "manual")]
    Manual,

    #[default]
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Origin {
    // Originally `type` in the JSON Schema
    origin_type: Option<OriginType>,
    reference: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Rom {
    file_name: Option<String>,
    embedded_title: Option<String>,
    description: Option<String>,
    // TODO: This should be a date of some kind
    release: Option<String>,
    platforms: Vec<Platform>,
    quirky_platforms: Option<HashMap<Platform, QuirkSet>>,
    authors: Option<Vec<String>>,
    // TODO: Support real images here
    images: Option<Vec<String>>,
    // TODO: Use an appropriate URL type here
    urls: Option<Vec<String>>,
    tickrate: Option<usize>,
    start_address: Option<u16>,
    screen_rotation: Option<ScreenRotation>,
    keys: Option<HashMap<Keymap, u8>>,
    touch_input_mode: Option<TouchInputMode>,
    font_style: Option<FontStyle>,
    colors: Option<Colors>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Platform {
    #[serde(rename = "originalChip8")]
    OriginalChip8,

    #[serde(rename = "hybridVIP")]
    HybridVIP,

    #[serde(rename = "modernChip8")]
    ModernChip8,

    #[serde(rename = "chip48")]
    Chip48,

    #[serde(rename = "superchip1")]
    Superchip1,

    #[serde(rename = "superchip")]
    Superchip,

    #[serde(rename = "xochip")]
    XOChip,

    #[serde(rename = "chip8X")]
    Chip8X,

    #[serde(rename = "megachip8")]
    MegaChip8,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum ScreenRotation {
    #[default]
    Landscape = 0,
    Portrait = 90,
    LandscapeFlipped = 180,
    PortraitFlipped = 270,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Keymap {
    #[serde(rename = "up")]
    P1Up,

    #[serde(rename = "down")]
    P1Down,

    #[serde(rename = "left")]
    P1Left,

    #[serde(rename = "right")]
    P1Right,

    #[serde(rename = "a")]
    P1A,

    #[serde(rename = "b")]
    P1B,

    #[serde(rename = "player2Up")]
    P2Up,

    #[serde(rename = "player2Down")]
    P2Down,

    #[serde(rename = "player2Left")]
    P2Left,

    #[serde(rename = "player2Right")]
    P2Right,

    #[serde(rename = "player2A")]
    P2A,

    #[serde(rename = "player2B")]
    P2B,
}

#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum TouchInputMode {
    #[default]
    #[serde(rename = "none")]
    None,

    #[serde(rename = "swipe")]
    Swipe,

    #[serde(rename = "seg16")]
    Seg16,

    #[serde(rename = "seg16fill")]
    Seg16Fill,

    #[serde(rename = "gamepad")]
    Gamepad,

    #[serde(rename = "vip")]
    VIP,
}

#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum FontStyle {
    #[default]
    #[serde(rename = "vip")]
    VIP,

    #[serde(rename = "octo")]
    Octo,

    #[serde(rename = "schip")]
    SCHIP,

    #[serde(rename = "dream6800")]
    Dream6800,

    #[serde(rename = "eti660")]
    ETI660,

    #[serde(rename = "fish")]
    Fish,

    #[serde(rename = "akouz1")]
    Akouz1,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct QuirkSet {
    shift: bool,
    memory_increment_by_x: bool,
    memory_leave_i_unchanged: bool,
    wrap: bool,
    jump: bool,
    vblank: bool,
    logic: bool,
}

// TODO: Better color type than strings here
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Colors {
    pixels: Option<Vec<String>>,
    buzzer: Option<String>,
    silence: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    mod serde {
        use super::*;
        use serde_json::Result;

        #[test]
        fn program_roundtrip() -> Result<()> {
            let input = r##"{
                "title": "Test Program",
                "origin": {
                    "type": "manual",
                    "reference": "What's this supposed to be?"
                },
                "description": "A description of the program",
                "release": "2023-06-24",
                "copyright": "Probably copyrighted or something",
                "license": "MIT",
                "authors": ["Someone"],
                "images": ["https://example.com/chip8/test-program.png"],
                "urls": ["https://example.com/chip8/test-program.html"],
                "roms": {
                    "0123456789abcdef0123456789abcdef01234567": {
                        "file": "test-program.ch8",
                        "embeddedTitle": "Test Program Embedded",
                        "description": "The test program to test all programs",
                        "release": "2023-06-24",
                        "platforms": ["originalChip8"],
                        "quirkyPlatforms": {
                            "originalChip8": {
                                "shift": false,
                                "memoryIncrementByX": false,
                                "memoryLeaveIUnchanged": false,
                                "wrap": false,
                                "jump": false,
                                "vblank": false,
                                "logic": false
                            }
                        },
                        "authors": ["Someone Else"],
                        "images": ["https://example.com/chip8/test-program-detail.png"],
                        "urls": ["https://example.com/chip8/test-program.ch8"],
                        "tickrate": 10,
                        "startAddress": 512,
                        "screenRotation": 0,
                        "keys": {
                            "up": 0,
                            "down": 1,
                            "left": 2,
                            "right": 3,
                            "a": 4,
                            "b": 5,
                            "player2Up": 16,
                            "player2Down": 17,
                            "player2Left": 18,
                            "player2Right": 19,
                            "player2A": 20,
                            "player2B": 21
                        },
                        "touchInputMode": "none",
                        "fontStyle": "vip",
                        "colors": {
                            "pixels": ["#000000", "#ff0000", "#00ff00", "#0000ff"],
                            "buzzer": "#cccccc",
                            "silence": "#555555"
                        }
                    }
                }
            }"##;

            let program: Program = serde_json::from_str(input)?;

            assert_eq!(program.title, "Test Program");

            Ok(())
        }
    }
}
