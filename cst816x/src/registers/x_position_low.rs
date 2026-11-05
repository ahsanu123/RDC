use bilge::prelude::*;

/// XposL, 0x04: `X[7:0]` (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct XPositionLow {
    pub coordinate: u8,
}
