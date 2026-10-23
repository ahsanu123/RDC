use bilge::prelude::*;

/// Internal control 0, write-only; command bits self-clear (Rev A, pages 14-15).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Default, Clone, Copy, PartialEq, Eq)]
pub struct Control0 {
    pub take_magnetic_measurement: bool,
    pub take_temperature_measurement: bool,
    pub measurement_interrupt_enabled: bool,
    pub set: bool,
    pub reset: bool,
    pub automatic_set_reset_enabled: bool,
    pub read_otp: bool,
    pub reserved: bool,
}
