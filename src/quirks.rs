use chip8_db::{platform::Platform, quirk::Quirk, Database};

use crate::DB;

/// The selected quirks that should be used for this ROM.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Quirks {
    /// Whether VF is reset on AND/OR/XOR instructions.
    pub vf_reset: bool,

    /// Whether DXYN instructions wait for the next frame before they occur.
    pub display_wait: bool,

    /// Whether shift instructions ignore VY.
    pub shift: bool,

    // Whether to increment I by the value of X, instead of the default behavior of X + 1.
    pub memory_increment_by_x: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for Quirks {
    fn default() -> Self {
        Quirks {
            vf_reset: false,
            display_wait: false,
            shift: false,
            memory_increment_by_x: false,
        }
    }
}

impl From<&Platform> for Quirks {
    fn from(value: &Platform) -> Self {
        let platform = DB
            .get_or_init(Database::new)
            .platforms
            .iter()
            .find(|platform| platform.id == *value)
            .expect("No matching platform is an error in chip-8-database");

        // TODO: Read quirkyPlatforms

        Self {
            vf_reset: *platform.quirks.get(&Quirk::Logic).unwrap_or(&false),
            display_wait: *platform.quirks.get(&Quirk::VBlank).unwrap_or(&false),
            shift: *platform.quirks.get(&Quirk::Shift).unwrap_or(&false),
            memory_increment_by_x: *platform
                .quirks
                .get(&Quirk::MemoryIncrementByX)
                .unwrap_or(&false),
        }
    }
}
