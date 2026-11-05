use bilge::prelude::*;

/// BPC0L, 0xB1: `BPC0[7:0]` (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct Bpc0Low {
    pub low_byte: u8,
}
