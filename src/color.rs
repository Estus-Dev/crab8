/// Parse a string in format `#rrggbb` to an array of u8 RGBA channels, hardcoding full alpha.
///
/// This is only intended for use on color strings from [chip8_db].
pub(crate) fn parse_color(color: &str) -> Result<[u8; 4], std::num::ParseIntError> {
    Ok([
        u8::from_str_radix(&color[1..3], 16)?,
        u8::from_str_radix(&color[3..5], 16)?,
        u8::from_str_radix(&color[5..7], 16)?,
        255,
    ])
}
