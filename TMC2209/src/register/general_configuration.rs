use bilge::prelude::*;

/// GCONF, general configuration. Datasheet rev. 1.09, section 5.1, page 23.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct GeneralConfiguration {
    pub analog_current_scaling: bool,
    pub internal_sense_resistors: bool,
    pub spread_cycle_enabled: bool,
    pub reversed_direction: bool,
    pub index_temperature_warning: bool,
    pub index_step_pulses: bool,
    pub power_down_pin_disabled: bool,
    pub register_microstep_selection: bool,
    pub step_filter_enabled: bool,
    pub test_mode: bool,
    pub unused: u22,
}

super::register!(GeneralConfiguration, 0x00, None);
impl super::ReadableRegister for GeneralConfiguration {}
impl super::WritableRegister for GeneralConfiguration {
    const WRITE_MASK: u32 = 0x0000_03ff;
}
