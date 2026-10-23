use bilge::prelude::*;

/// MODE, datasheet Rev. 7, figure 8. QuickStart is a self-clearing command.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Mode {
    pub unused_low: u12,
    pub hibernating: bool,
    pub sleep_enabled: bool,
    pub quick_start: bool,
    pub unused_high: bool,
}
super::readable!(Mode, 0x06);
