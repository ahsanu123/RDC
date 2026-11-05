use bilge::prelude::*;

/// BPC0H, 0xB0: `BPC0[15:8]` (CST816S register declaration, pages 1-2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct Bpc0High {
    pub high_byte: u8,
}
