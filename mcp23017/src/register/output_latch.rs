use bilge::prelude::*;

/// OLAT: output latches, independent of the actual input levels.
/// Datasheet: DS20001952D, section 3.5.11, Output Latch Register.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct OutputLatch {
    pub pin0: bool,
    pub pin1: bool,
    pub pin2: bool,
    pub pin3: bool,
    pub pin4: bool,
    pub pin5: bool,
    pub pin6: bool,
    pub pin7: bool,
}

super::register!(OutputLatch, 0x14, 0x00);
impl super::WritableRegister for OutputLatch {}
