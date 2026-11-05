use bilge::prelude::*;

/// XposH, 0x03: `X[11:8]` (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct XPositionHigh {
    pub coordinate: u4,
    pub unspecified: u4,
}
