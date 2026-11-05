use bilge::prelude::*;

/// BPC1H, 0xB2: `BPC1[15:8]` (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct Bpc1High {
    pub high_byte: u8,
}
