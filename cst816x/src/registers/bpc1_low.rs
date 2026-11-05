use bilge::prelude::*;

/// BPC1L, 0xB3: `BPC1[7:0]` (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct Bpc1Low {
    pub low_byte: u8,
}
