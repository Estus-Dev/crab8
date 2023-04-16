/// The CHIP-8 has 16 general-purpose registers, V0-VF.
/// Each register is 8 bits wide.
/// https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#data-registers
#[derive(Clone, Copy, Debug)]
#[allow(non_snake_case)]
pub enum Register {
    /// V0 is a general use register.
    V0 = 0x00,
    /// V1 is a general use register.
    V1 = 0x01,
    /// V2 is a general use register.
    V2 = 0x02,
    /// V3 is a general use register.
    V3 = 0x03,
    /// V4 is a general use register.
    V4 = 0x04,
    /// V5 is a general use register.
    V5 = 0x05,
    /// V6 is a general use register.
    V6 = 0x06,
    /// V7 is a general use register.
    V7 = 0x07,
    /// V8 is a general use register.
    V8 = 0x08,
    /// V9 is a general use register.
    V9 = 0x09,
    /// VA is a general use register.
    VA = 0x0A,
    /// VB is a general use register.
    VB = 0x0B,
    /// VC is a general use register.
    VC = 0x0C,
    /// VD is a general use register.
    VD = 0x0D,
    /// VE is a general use register.
    VE = 0x0E,
    /// VF is a general use reigster that is often used as a flag.
    VF = 0x0F,
}

impl TryFrom<i32> for Register {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use Register::*;

        match value {
            i if i == V0 as i32 => Ok(V0),
            i if i == V1 as i32 => Ok(V1),
            i if i == V2 as i32 => Ok(V2),
            i if i == V2 as i32 => Ok(V2),
            i if i == V3 as i32 => Ok(V3),
            i if i == V4 as i32 => Ok(V4),
            i if i == V5 as i32 => Ok(V5),
            i if i == V6 as i32 => Ok(V6),
            i if i == V7 as i32 => Ok(V7),
            i if i == V8 as i32 => Ok(V8),
            i if i == V9 as i32 => Ok(V9),
            i if i == VA as i32 => Ok(VA),
            i if i == VB as i32 => Ok(VB),
            i if i == VC as i32 => Ok(VC),
            i if i == VD as i32 => Ok(VD),
            i if i == VE as i32 => Ok(VE),
            i if i == VF as i32 => Ok(VF),

            // TODO: Use a proper error
            _ => Err(()),
        }
    }
}
