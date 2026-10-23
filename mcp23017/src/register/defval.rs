use bilge::prelude::*;

/// DEFVAL: comparison levels for interrupts when INTCON is set.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Defval {
    pub pin0: bool,
    pub pin1: bool,
    pub pin2: bool,
    pub pin3: bool,
    pub pin4: bool,
    pub pin5: bool,
    pub pin6: bool,
    pub pin7: bool,
}

super::register!(Defval, 0x06, 0x00);
impl super::WritableRegister for Defval {}
