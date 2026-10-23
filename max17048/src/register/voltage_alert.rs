use bilge::prelude::*;

/// VALRT, datasheet Rev. 7, figure 11: 20 mV/count.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct VoltageAlert {
    pub maximum: u8,
    pub minimum: u8,
}
super::readable!(VoltageAlert, 0x14);
