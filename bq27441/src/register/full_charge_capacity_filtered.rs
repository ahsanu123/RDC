use bilge::prelude::*;

/// FullChargeCapacityFiltered() in mAh; SLUUAC9A section 4.19.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct FullChargeCapacityFiltered {
    pub milliamp_hours: u16,
}

super::register!(FullChargeCapacityFiltered, 0x2e, None);
