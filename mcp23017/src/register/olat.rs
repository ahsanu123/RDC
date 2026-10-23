use bilge::prelude::*;

/// OLAT: output latches, independent of the actual input levels.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Olat {
    pub pin0: bool,
    pub pin1: bool,
    pub pin2: bool,
    pub pin3: bool,
    pub pin4: bool,
    pub pin5: bool,
    pub pin6: bool,
    pub pin7: bool,
}

super::register!(Olat, 0x14, 0x00);
impl super::WritableRegister for Olat {}
