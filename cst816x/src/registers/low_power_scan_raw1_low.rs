use bilge::prelude::*;

/// LpScanRaw1L, 0xF1: channel 1 reference low byte (CST816S declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanRaw1Low {
    pub low_byte: u8,
}
