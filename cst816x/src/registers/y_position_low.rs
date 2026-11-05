use bilge::prelude::*;

/// YposL, 0x06: `Y[7:0]` (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct YPositionLow {
    pub coordinate: u8,
}
