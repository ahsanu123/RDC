use bilge::prelude::*;

use crate::FaultCycle;

/// Notch filter selection (Rev 3, Table 2, page 13).
#[bitsize(1)]
#[derive(FromBits, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MainsFilter {
    #[default]
    Hertz60 = 0,
    Hertz50 = 1,
}

/// Configuration 00h; fields are listed least-significant first (Rev 3, Table 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Default, Clone, Copy, PartialEq, Eq)]
pub struct Configuration {
    pub mains_filter: MainsFilter,
    pub clear_fault_status: bool,
    pub fault_detection_cycle: FaultCycle,
    pub three_wire_compensation: bool,
    pub one_shot: bool,
    pub automatic_conversion: bool,
    pub bias_enabled: bool,
}

impl Configuration {
    pub(crate) fn stable(mut self) -> Self {
        self.set_one_shot(false);
        self.set_fault_detection_cycle(FaultCycle::None);
        self.set_clear_fault_status(false);
        self
    }
}
