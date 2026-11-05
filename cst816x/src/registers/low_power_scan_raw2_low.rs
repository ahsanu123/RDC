use bilge::prelude::*;

/// LpScanRaw2L, 0xF3: channel 2 reference low byte (CST816S declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanRaw2Low {
    pub low_byte: u8,
}
