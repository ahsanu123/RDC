use bilge::prelude::*;

/// LpScanRaw1H, 0xF0: channel 1 reference high byte (CST816S declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanRaw1High {
    pub high_byte: u8,
}
