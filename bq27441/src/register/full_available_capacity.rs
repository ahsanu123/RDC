use bilge::prelude::*;

/// FullAvailableCapacity() in mAh; SLUUAC9A section 4.6.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct FullAvailableCapacity {
    pub milliamp_hours: u16,
}

super::register!(FullAvailableCapacity, 0x0a, None);
