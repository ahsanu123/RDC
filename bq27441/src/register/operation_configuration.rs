use bilge::prelude::*;

/// OpConfig() read-only mirror; SLUUAC9A sections 5.1 and 6.4.1.5.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct OperationConfiguration {
    pub external_temperature: bool,
    pub reserved1: bool,
    pub battery_low_output: bool,
    pub reserved3: bool,
    pub remaining_capacity_tracks_full: bool,
    pub sleep_enabled: bool,
    pub reserved6: u2,
    pub reserved8: bool,
    pub reserved9: bool,
    pub reserved10: bool,
    pub output_active_high: bool,
    pub battery_insert_pull_up: bool,
    pub battery_insert_enabled: bool,
    pub reserved14: u2,
}

super::register!(OperationConfiguration, 0x3a, Some(0x25f8));
