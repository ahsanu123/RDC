use bilge::prelude::*;

/// INTCAP: port state latched at interrupt time; reading clears that port's interrupt.
/// Datasheet: DS20001952C, section 3.5.9, Interrupt Captured Register.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct InterruptCapture {
    pub pin0: bool,
    pub pin1: bool,
    pub pin2: bool,
    pub pin3: bool,
    pub pin4: bool,
    pub pin5: bool,
    pub pin6: bool,
    pub pin7: bool,
}

super::register!(InterruptCapture, 0x10, 0x00);
