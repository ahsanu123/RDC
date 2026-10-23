use bilge::prelude::*;

/// INTCON: true compares against DEFVAL, false against the previous input.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Intcon {
    pub pin0: bool,
    pub pin1: bool,
    pub pin2: bool,
    pub pin3: bool,
    pub pin4: bool,
    pub pin5: bool,
    pub pin6: bool,
    pub pin7: bool,
}

super::register!(Intcon, 0x08, 0x00);
impl super::WritableRegister for Intcon {}
