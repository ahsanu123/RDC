use bilge::prelude::*;

/// PWM_SCALE, amplitude plus signed 9-bit automatic correction. Rev. 1.09, section 5.5.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct PwmScaling {
    pub amplitude: u8,
    pub unused_low: u8,
    pub automatic_correction_bits: u9,
    pub unused_high: u7,
}

super::register!(PwmScaling, 0x71, None);
impl super::ReadableRegister for PwmScaling {}
