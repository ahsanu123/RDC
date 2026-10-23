use bilge::prelude::*;

/// PWM_AUTO, learned offset and gradient. Rev. 1.09, section 5.5, page 32.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct AutomaticPwmValues {
    pub amplitude_offset: u8,
    pub unused_low: u8,
    pub amplitude_gradient: u8,
    pub unused_high: u8,
}

super::register!(AutomaticPwmValues, 0x72, None);
impl super::ReadableRegister for AutomaticPwmValues {}
