use bilge::prelude::*;

/// Valid MRES encodings (0=256, 8=full step). Rev. 1.09, section 5.5.1.
#[bitsize(4)]
#[derive(Clone, Copy, Debug, TryFromBits, PartialEq, Eq)]
pub enum MicrostepResolution {
    Microsteps256,
    Microsteps128,
    Microsteps64,
    Microsteps32,
    Microsteps16,
    Microsteps8,
    Microsteps4,
    Microsteps2,
    FullStep,
}

/// CHOPCONF, driver enable and chopper configuration. Rev. 1.09, section 5.5.1, pages 33-34.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct ChopperConfiguration {
    pub off_time: u4,
    pub hysteresis_start: u3,
    pub hysteresis_end: u4,
    pub unused_low: u4,
    pub blank_time: u2,
    pub high_sense_sensitivity: bool,
    pub unused_middle: u6,
    pub microstep_resolution: u4,
    pub interpolation_enabled: bool,
    pub double_edge_steps: bool,
    pub ground_short_protection_disabled: bool,
    pub low_side_short_protection_disabled: bool,
}

super::register!(ChopperConfiguration, 0x6c, None);
impl super::ReadableRegister for ChopperConfiguration {}
impl super::WritableRegister for ChopperConfiguration {
    const WRITE_MASK: u32 = 0xff03_87ff;
}
