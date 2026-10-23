use bilge::prelude::*;

/// RemainingCapacityFiltered() in mAh; SLUUAC9A section 4.17.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct RemainingCapacityFiltered {
    pub milliamp_hours: u16,
}

super::register!(RemainingCapacityFiltered, 0x2a, None);
