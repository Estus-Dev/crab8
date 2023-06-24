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
    GameJam,
    Event,
    Magazine,
    Manual,
    #[default]
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
    OriginalChip8,
    HybridVIP,
    ModernChip8,
    Chip48,
    Superchip1,
    Superchip,
    XOChip,
    Chip8X,
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
    P1Up,
    P1Down,
    P1Left,
    P1Right,
    P1A,
    P1B,
    P2Up,
    P2Down,
    P2Left,
    P2Right,
    P2A,
    P2B,
}

#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum TouchInputMode {
    #[default]
    None,
    Swipe,
    Seg16,
    Seg16Fill,
    Gamepad,
    VIP,
}

#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum FontStyle {
    Octo,
    #[default]
    VIP,
    SCHIP,
    Dream6800,
    ETI660,
    Fish,
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
