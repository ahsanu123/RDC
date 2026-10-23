use bilge::prelude::*;

/// STATUS, datasheet Rev. 7, figure 13. Flags clear when written zero.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Status {
    pub unused_low: u8,
    pub reset_indicator: bool,
    pub voltage_high: bool,
    pub voltage_low: bool,
    pub voltage_reset: bool,
    pub state_of_charge_low: bool,
    pub state_of_charge_changed: bool,
    pub voltage_reset_alert_enabled: bool,
    pub unused_high: bool,
}
super::readable!(Status, 0x1a);
