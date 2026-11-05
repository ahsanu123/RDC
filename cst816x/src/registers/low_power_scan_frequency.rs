use bilge::prelude::*;

/// LpScanFreq, 0xF7 (CST816S register declaration, page 4).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanFrequency {
    /// Raw frequency setting: 1-255; smaller values increase sensitivity.
    pub frequency: u8,
}

impl Default for LowPowerScanFrequency {
    fn default() -> Self {
        Self::new(7)
    }
}
