use bilge::prelude::*;

/// GPPU: true enables the weak pull-up on an input.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Gppu {
    pub pin0: bool,
    pub pin1: bool,
    pub pin2: bool,
    pub pin3: bool,
    pub pin4: bool,
    pub pin5: bool,
    pub pin6: bool,
    pub pin7: bool,
}

super::register!(Gppu, 0x0c, 0x00);
impl super::WritableRegister for Gppu {}
