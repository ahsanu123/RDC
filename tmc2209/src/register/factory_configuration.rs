use bilge::prelude::*;

/// FACTORY_CONF, OTP-loaded calibration; preserve on ordinary setup. Rev. 1.09, section 5.1.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct FactoryConfiguration {
    pub clock_trim: u5,
    pub unused_low: u3,
    pub temperature_trim: u2,
    pub unused_high: u22,
}

super::register!(FactoryConfiguration, 0x07, None);
impl super::ReadableRegister for FactoryConfiguration {}
impl super::WritableRegister for FactoryConfiguration {
    const WRITE_MASK: u32 = 0x0000_031f;
}
