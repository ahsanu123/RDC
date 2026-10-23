use bilge::prelude::*;

/// INTCON: true compares against DEFVAL, false against the previous input.
/// Datasheet: DS20001952D, section 3.5.5, Interrupt Control Register.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct InterruptControl {
    pub pin0: bool,
    pub pin1: bool,
    pub pin2: bool,
    pub pin3: bool,
    pub pin4: bool,
    pub pin5: bool,
    pub pin6: bool,
    pub pin7: bool,
}

super::register!(InterruptControl, 0x08, 0x00);
impl super::WritableRegister for InterruptControl {}
