use bilge::prelude::*;

/// LpAutoWakeTime, 0xF4 (CST816S register declaration, pages 3-4).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerAutoWakeTime {
    /// Low-power automatic recalibration period; range: 1-5 minutes.
    pub minutes: u8,
}

impl Default for LowPowerAutoWakeTime {
    fn default() -> Self {
        Self::new(5)
    }
}
