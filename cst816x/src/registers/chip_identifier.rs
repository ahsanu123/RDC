use bilge::prelude::*;

/// ChipID, 0xA7 (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct ChipIdentifier {
    pub identifier: u8,
}
