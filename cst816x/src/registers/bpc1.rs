use bilge::prelude::*;

/// Combined BPC1H/BPC1L, 0xB2-0xB3 (CST816S register declaration, page 2).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct Bpc1 {
    pub value: u16,
}
