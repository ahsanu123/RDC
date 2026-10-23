use bilge::prelude::*;

/// CONFIG, datasheet Rev. 7, figure 10.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Configuration {
    pub empty_alert_threshold: u5,
    pub alert: bool,
    pub state_of_charge_change_alert: bool,
    pub sleeping: bool,
    pub compensation: u8,
}
super::readable!(Configuration, 0x0c);
