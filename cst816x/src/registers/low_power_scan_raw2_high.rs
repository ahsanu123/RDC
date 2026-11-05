use bilge::prelude::*;

/// LpScanRaw2H, 0xF2: channel 2 reference high byte (CST816S declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanRaw2High {
    pub high_byte: u8,
}
