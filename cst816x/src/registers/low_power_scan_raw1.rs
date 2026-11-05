use bilge::prelude::*;

/// Combined LpScanRaw1H/LpScanRaw1L, 0xF0-0xF1 (CST816S declaration, page 3).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanRaw1 {
    pub value: u16,
}
