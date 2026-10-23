use bilge::prelude::*;

/// NominalAvailableCapacity() in mAh; SLUUAC9A section 4.5.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct NominalAvailableCapacity {
    pub milliamp_hours: u16,
}

super::register!(NominalAvailableCapacity, 0x08, None);
