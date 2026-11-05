use bilge::prelude::*;

/// LongPressTime, 0xFC (CST816S register declaration, page 5).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LongPressTime {
    /// Reset after a long press for this time; 0 disables.
    pub seconds: u8,
}

impl Default for LongPressTime {
    fn default() -> Self {
        Self::new(10)
    }
}
