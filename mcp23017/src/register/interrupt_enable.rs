use bilge::prelude::*;

/// GPINTEN: true enables interrupt-on-change for an input.
/// Datasheet: DS20001952C, section 3.5.3, Interrupt-on-Change Control Register.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct InterruptEnable {
    pub pin0: bool,
    pub pin1: bool,
    pub pin2: bool,
    pub pin3: bool,
    pub pin4: bool,
    pub pin5: bool,
    pub pin6: bool,
    pub pin7: bool,
}

super::register!(InterruptEnable, 0x04, 0x00);
impl super::WritableRegister for InterruptEnable {}
