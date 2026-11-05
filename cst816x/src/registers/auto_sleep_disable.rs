use bilge::prelude::*;

/// DisAutoSleep, 0xFE (CST816S register declaration, page 5).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct AutoSleepDisable {
    /// Zero enables automatic sleep; any nonzero byte disables it.
    pub disable: u8,
}

impl Default for AutoSleepDisable {
    fn default() -> Self {
        Self::new(0)
    }
}
