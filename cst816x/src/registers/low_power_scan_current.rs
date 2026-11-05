use bilge::prelude::*;

/// LpScanIdac, 0xF8 (CST816S register declaration, page 4).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanCurrent {
    /// Raw current setting: 1-255; smaller values increase sensitivity.
    pub current: u8,
}
