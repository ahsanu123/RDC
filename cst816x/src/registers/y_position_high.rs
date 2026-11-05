use bilge::prelude::*;

/// YposH, 0x05: `Y[11:8]` (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct YPositionHigh {
    pub coordinate: u4,
    pub unspecified: u4,
}
