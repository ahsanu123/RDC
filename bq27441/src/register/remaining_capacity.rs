use bilge::prelude::*;

/// RemainingCapacity() in mAh; SLUUAC9A section 4.7.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct RemainingCapacity {
    pub milliamp_hours: u16,
}

super::register!(RemainingCapacity, 0x0c, None);
