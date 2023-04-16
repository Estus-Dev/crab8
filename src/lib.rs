/// The CHIP-8 has 16 general-purpose registers, V0-VF.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#data-registers
#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct Registers {
    V0: Register,
    V1: Register,
    V2: Register,
    V3: Register,
    V4: Register,
    V5: Register,
    V6: Register,
    V7: Register,
    V8: Register,
    V9: Register,
    VA: Register,
    VB: Register,
    VC: Register,
    VD: Register,
    VE: Register,
    /// VF is often used as a flag but can also be used for general use
    VF: Register,
}

/// General use registers on the CHIP-8 are 8-bits wide.
#[derive(Debug, Default)]
struct Register(u8);
