use bilge::prelude::*;

/// NorScanPer, 0xEE (CST816S register declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct NormalScanPeriod {
    /// Raw 10 ms units; range: 1-30. Affects auto-wake and auto-sleep timing.
    pub tens_of_milliseconds: u8,
}

impl Default for NormalScanPeriod {
    fn default() -> Self {
        Self::new(1)
    }
}
