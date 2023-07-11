use chip8_db::Metadata;

/// The selected quirks that should be used for this ROM.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Quirks {
    /// Whether VF is reset on AND/OR/XOR instructions.
    pub vf_reset: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for Quirks {
    fn default() -> Self {
        Quirks { vf_reset: false }
    }
}
