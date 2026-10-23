use bilge::prelude::*;

/// IOIN, pin levels and silicon identity (0x21). Rev. 1.09, section 5.1, pages 24-25.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct InputStatus {
    pub hardware_disabled: bool,
    pub unused_bit1: bool,
    pub address_pin_zero: bool,
    pub address_pin_one: bool,
    pub diagnostic: bool,
    pub unused_bit5: bool,
    pub uart_pin: bool,
    pub step_pin: bool,
    pub spread_cycle_pin: bool,
    pub direction_pin: bool,
    pub unused_middle: u14,
    pub version: u8,
}

super::register!(InputStatus, 0x06, None);
impl super::ReadableRegister for InputStatus {}
