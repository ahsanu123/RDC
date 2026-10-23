use bilge::prelude::*;

/// Flags() operating status; SLUUAC9A section 4.4 and table 4-4.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Flags {
    pub discharging: bool,
    pub final_charge_warning: bool,
    pub low_charge_warning: bool,
    pub battery_detected: bool,
    pub configuration_update: bool,
    pub power_on_reset: bool,
    pub reserved6: bool,
    pub open_circuit_voltage_taken: bool,
    pub charging_allowed: bool,
    pub fully_charged: bool,
    pub reserved10: u4,
    pub under_temperature: bool,
    pub over_temperature: bool,
}

super::register!(Flags, 0x06, None);
