use bilge::prelude::*;

/// Big-endian high (03h-04h) or low (05h-06h) threshold (Rev 3, Table 6).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Default, Clone, Copy, PartialEq, Eq)]
pub struct FaultThreshold {
    pub ignored: bool,
    pub count: u15,
}
