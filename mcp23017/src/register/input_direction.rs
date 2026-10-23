use bilge::prelude::*;

/// Pin direction: reset leaves every pin as an input.
#[bitsize(1)]
#[derive(Clone, Copy, Debug, FromBits, PartialEq, Eq)]
pub enum Direction {
    Output,
    Input,
}

/// IODIR: one direction bit per pin.
/// Datasheet: DS20001952C, section 3.5.1, I/O Direction Register.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct InputDirection {
    pub pin0: Direction,
    pub pin1: Direction,
    pub pin2: Direction,
    pub pin3: Direction,
    pub pin4: Direction,
    pub pin5: Direction,
    pub pin6: Direction,
    pub pin7: Direction,
}

super::register!(InputDirection, 0x00, 0xff);
impl super::WritableRegister for InputDirection {}
