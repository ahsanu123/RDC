use bilge::prelude::*;

/// Combined XposH/XposL, 0x03-0x04 (CST816S register declaration, page 1).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct XPosition {
    pub coordinate: u12,
    pub unspecified: u4,
}
