use bilge::prelude::*;

/// Combined BPC0H/BPC0L, 0xB0-0xB1 (CST816S register declaration, pages 1-2).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct Bpc0 {
    pub value: u16,
}
