use bilge::prelude::*;

/// DataBlock() 32-byte page selector; SLUUAC9A section 5.4.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct DataBlock {
    pub number: u8,
}
