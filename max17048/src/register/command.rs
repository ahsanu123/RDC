use bilge::prelude::*;

/// CMD, datasheet Rev. 7, page 14.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Command {
    pub value: u16,
}
impl Command {
    /// Power-on reset; the device does not acknowledge the final byte.
    pub const POWER_ON_RESET: u16 = 0x5400;
}
super::readable!(Command, 0xfe);
