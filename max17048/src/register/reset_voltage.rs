use bilge::prelude::*;

/// Dis field encodings, datasheet Rev. 7, figure 12.
#[bitsize(1)]
#[derive(Clone, Copy, Debug, FromBits, PartialEq, Eq)]
pub enum ResetComparator {
    Enabled = 0,
    Disabled = 1,
}

/// VRESET/ID, datasheet Rev. 7, figure 12.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct ResetVoltage {
    pub identifier: u8,
    pub comparator: ResetComparator,
    pub threshold: u7,
}
super::readable!(ResetVoltage, 0x18);
