use bilge::prelude::*;

/// AutoReset / DebounceTime, 0xFB (CST816S register declaration, page 5).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct AutoReset {
    /// Reset after touch without a valid gesture for this time; 0 disables.
    pub seconds: u8,
}
