use bilge::prelude::*;

/// GPIO: pin levels on read, output latch on write. Reads clear the port interrupt.
/// Datasheet: DS20001952C, section 3.5.10, Port Register.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct GeneralPurposeInputOutput {
    pub pin0: bool,
    pub pin1: bool,
    pub pin2: bool,
    pub pin3: bool,
    pub pin4: bool,
    pub pin5: bool,
    pub pin6: bool,
    pub pin7: bool,
}

super::register!(GeneralPurposeInputOutput, 0x12, 0x00);
impl super::WritableRegister for GeneralPurposeInputOutput {}
