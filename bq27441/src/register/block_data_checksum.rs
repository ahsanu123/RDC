use bilge::prelude::*;

/// BlockDataChecksum(); SLUUAC9A section 5.6.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct BlockDataChecksum {
    pub value: u8,
}
