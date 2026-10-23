use bilge::prelude::*;

/// SGTHRS, stall when SG_RESULT <= twice this value. Rev. 1.09, section 5.3, page 29.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct StallDetectionThreshold {
    pub threshold: u8,
    pub unused: u24,
}

super::register!(StallDetectionThreshold, 0x40, Some(0));
impl super::WritableRegister for StallDetectionThreshold {
    const WRITE_MASK: u32 = 0x0000_00ff;
}
