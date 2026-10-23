use bilge::prelude::*;

/// DRV_STATUS, faults and actual current scale. Rev. 1.09, section 5.5.3, page 37.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct DriverStatus {
    pub temperature_warning: bool,
    pub temperature_shutdown: bool,
    pub ground_short_phase_a: bool,
    pub ground_short_phase_b: bool,
    pub low_side_short_phase_a: bool,
    pub low_side_short_phase_b: bool,
    pub open_load_phase_a: bool,
    pub open_load_phase_b: bool,
    pub temperature_above_120: bool,
    pub temperature_above_143: bool,
    pub temperature_above_150: bool,
    pub temperature_above_157: bool,
    pub unused_low: u4,
    pub actual_current_scale: u5,
    pub unused_high: u9,
    pub stealth_chop_active: bool,
    pub standstill: bool,
}

super::register!(DriverStatus, 0x6f, None);
impl super::ReadableRegister for DriverStatus {}
