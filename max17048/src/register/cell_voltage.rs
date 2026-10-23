use bilge::prelude::*;

/// VCELL, datasheet Rev. 7, page 10: 78.125 uV/count on MAX17048.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct CellVoltage {
    pub count: u16,
}
super::readable!(CellVoltage, 0x02);
