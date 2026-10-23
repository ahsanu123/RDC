use bilge::prelude::*;

/// DEFVAL: comparison levels for interrupts when INTCON is set.
/// Datasheet: DS20001952C, section 3.5.4, Default Compare Register for Interrupt-on-Change.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct InterruptDefaultValue {
    pub pin0: bool,
    pub pin1: bool,
    pub pin2: bool,
    pub pin3: bool,
    pub pin4: bool,
    pub pin5: bool,
    pub pin6: bool,
    pub pin7: bool,
}

super::register!(InterruptDefaultValue, 0x06, 0x00);
impl super::WritableRegister for InterruptDefaultValue {}
