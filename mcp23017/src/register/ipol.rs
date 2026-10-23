use bilge::prelude::*;

/// IPOL: true inverts the corresponding input in GPIO reads.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Ipol {
    pub pin0: bool,
    pub pin1: bool,
    pub pin2: bool,
    pub pin3: bool,
    pub pin4: bool,
    pub pin5: bool,
    pub pin6: bool,
    pub pin7: bool,
}

super::register!(Ipol, 0x02, 0x00);
impl super::WritableRegister for Ipol {}
