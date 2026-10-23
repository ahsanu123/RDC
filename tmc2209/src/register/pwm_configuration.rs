use bilge::prelude::*;

/// FREEWHEEL behavior at zero hold-current setting. Rev. 1.09, section 5.5.2.
#[bitsize(2)]
#[derive(Clone, Copy, Debug, FromBits, PartialEq, Eq)]
pub enum StandstillMode {
    Normal,
    Freewheeling,
    LowSideBraking,
    HighSideBraking,
}

/// PWMCONF, StealthChop control. Rev. 1.09, section 5.5.2, pages 35-36.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct PwmConfiguration {
    pub amplitude_offset: u8,
    pub amplitude_gradient: u8,
    pub frequency: u2,
    pub automatic_amplitude: bool,
    pub automatic_gradient: bool,
    pub standstill_mode: StandstillMode,
    pub unused: u2,
    pub regulation_speed: u4,
    pub amplitude_limit: u4,
}

super::register!(PwmConfiguration, 0x70, None);
impl super::ReadableRegister for PwmConfiguration {}
impl super::WritableRegister for PwmConfiguration {
    const WRITE_MASK: u32 = 0xff3f_ffff;
}
