use bilge::prelude::*;

/// VERSION, datasheet Rev. 7, page 11.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Version {
    pub production_version: u16,
}
super::readable!(Version, 0x08);
