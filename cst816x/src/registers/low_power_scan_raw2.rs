use bilge::prelude::*;

/// Combined LpScanRaw2H/LpScanRaw2L, 0xF2-0xF3 (CST816S declaration, page 3).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanRaw2 {
    pub value: u16,
}
