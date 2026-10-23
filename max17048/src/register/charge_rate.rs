use bilge::prelude::*;

/// CRATE, datasheet Rev. 7, page 13: signed 0.208 %/hour/count.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct ChargeRate {
    pub count: u16,
}
super::readable!(ChargeRate, 0x16);
