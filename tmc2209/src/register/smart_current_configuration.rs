use bilge::prelude::*;

/// COOLCONF, adaptive current control. Rev. 1.09, section 5.3.1, page 30.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct SmartCurrentConfiguration {
    pub minimum_load: u4,
    pub unused_bit4: bool,
    pub current_increment: u2,
    pub unused_bit7: bool,
    pub load_hysteresis: u4,
    pub unused_bit12: bool,
    pub current_decrease_speed: u2,
    pub quarter_minimum_current: bool,
    pub unused_high: u16,
}

super::register!(SmartCurrentConfiguration, 0x42, Some(0));
impl super::WritableRegister for SmartCurrentConfiguration {
    const WRITE_MASK: u32 = 0x0000_ef6f;
}
