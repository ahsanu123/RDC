use bilge::prelude::*;

/// SOC, datasheet Rev. 7, page 10: Q8.8 percent.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct StateOfCharge {
    pub fractional: u8,
    pub percent: u8,
}
super::readable!(StateOfCharge, 0x04);
