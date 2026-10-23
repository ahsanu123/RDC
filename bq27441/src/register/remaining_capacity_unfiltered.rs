use bilge::prelude::*;

/// RemainingCapacityUnfiltered() in mAh; SLUUAC9A section 4.16.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct RemainingCapacityUnfiltered {
    pub milliamp_hours: u16,
}

super::register!(RemainingCapacityUnfiltered, 0x28, None);
