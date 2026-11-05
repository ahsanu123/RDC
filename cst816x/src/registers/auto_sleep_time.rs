use bilge::prelude::*;

/// AutoSleepTime, 0xF9 (CST816S register declaration, page 4).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct AutoSleepTime {
    /// Time without touch before entering low-power mode.
    pub seconds: u8,
}

impl Default for AutoSleepTime {
    fn default() -> Self {
        Self::new(2)
    }
}
