use chip8_db::{platform::Platform, quirk::Quirk, Database};

use crate::DB;

/// The selected quirks that should be used for this ROM.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Quirks {
    /// Whether VF is reset on AND/OR/XOR instructions.
    pub vf_reset: bool,

    /// Whether DXYN instructions wait for the next frame before they occur.
    pub display_wait: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for Quirks {
    fn default() -> Self {
        Quirks {
            vf_reset: false,
            display_wait: false,
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
            vf_reset: platform.quirks[&Quirk::Logic],
            display_wait: platform.quirks[&Quirk::VBlank],
        }
    }
}
