use bilge::prelude::*;

/// LpScanTH, 0xF5 (CST816S register declaration, page 4).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanThreshold {
    /// Wake-up threshold; range: 1-255. Smaller values increase sensitivity.
    pub threshold: u8,
}

impl Default for LowPowerScanThreshold {
    fn default() -> Self {
        Self::new(48)
    }
}
