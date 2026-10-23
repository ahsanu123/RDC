use bilge::prelude::*;

/// OpConfig() read-only mirror; SLUUAC9A sections 5.1 and 6.4.1.5.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct OperationConfiguration {
    pub external_temperature: bool,
    pub reserved1: bool,
    pub battery_insert_pull_up: bool,
    pub battery_insert_enabled: bool,
    pub battery_low_output: bool,
    pub reserved5: u2,
    pub output_active_high: bool,
    pub sleep_enabled: bool,
    pub reserved9: u7,
}

super::register!(OperationConfiguration, 0x3a, Some(0x25f8));
