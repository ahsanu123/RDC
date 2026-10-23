use bilge::prelude::*;

/// BlockDataControl(); write zero to enable RAM access (SLUUAC9A section 5.7).
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct BlockDataControl {
    pub value: u8,
}
