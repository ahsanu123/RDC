use bilge::prelude::*;

/// VRESET/ID, datasheet Rev. 7, figure 12.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct ResetVoltage {
    pub identifier: u8,
    pub comparator_disabled: bool,
    pub threshold: u7,
}
super::readable!(ResetVoltage, 0x18);
