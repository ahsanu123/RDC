use bilge::prelude::*;

/// ProjID, 0xA8 (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct ProjectIdentifier {
    pub identifier: u8,
}
