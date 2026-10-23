use bilge::prelude::*;

/// Big-endian 01h-02h pair: fault bit and 15-bit ratio (Rev 3, Tables 4-5).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Default, Clone, Copy, PartialEq, Eq)]
pub struct RtdResistance {
    pub fault: bool,
    pub count: u15,
}
