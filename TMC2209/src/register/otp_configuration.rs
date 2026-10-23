use bilge::prelude::*;

/// OTP_READ; shared fields depend on spread_cycle_default. Rev. 1.09, section 5.1.1.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct OtpConfiguration {
    pub clock_trim: u5,
    pub temperature_trim: bool,
    pub internal_sense_resistors: bool,
    pub blank_time: bool,
    pub pwm_gradient_or_off_time: u4,
    pub automatic_gradient_or_hysteresis_start: bool,
    pub pwm_threshold_or_hysteresis: u3,
    pub pwm_offset_or_hysteresis_end: bool,
    pub regulation_speed: bool,
    pub pwm_frequency: bool,
    pub hold_delay: u2,
    pub hold_current: u2,
    pub spread_cycle_default: bool,
    pub unused: u8,
}

super::register!(OtpConfiguration, 0x05, None);
impl super::ReadableRegister for OtpConfiguration {}
