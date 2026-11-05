use bilge::prelude::*;

/// LpScanWin, 0xF6 (CST816S register declaration, page 4).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanWindow {
    /// Raw measurement range: 0-3. Larger values increase sensitivity and power.
    pub window: u8,
}

impl Default for LowPowerScanWindow {
    fn default() -> Self {
        Self::new(3)
    }
}
