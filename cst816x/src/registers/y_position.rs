use bilge::prelude::*;

/// Combined YposH/YposL, 0x05-0x06 (CST816S register declaration, page 1).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct YPosition {
    pub coordinate: u12,
    pub unspecified: u4,
}
