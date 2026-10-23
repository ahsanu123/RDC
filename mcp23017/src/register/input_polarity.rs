use bilge::prelude::*;

/// IPOL: true inverts the corresponding input in GPIO reads.
/// Datasheet: DS20001952D, section 3.5.2, Input Polarity Register.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct InputPolarity {
    pub pin0: bool,
    pub pin1: bool,
    pub pin2: bool,
    pub pin3: bool,
    pub pin4: bool,
    pub pin5: bool,
    pub pin6: bool,
    pub pin7: bool,
}

super::register!(InputPolarity, 0x02, 0x00);
impl super::WritableRegister for InputPolarity {}
