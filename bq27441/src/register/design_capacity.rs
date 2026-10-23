use bilge::prelude::*;

/// DesignCapacity() read-only mirror in mAh; SLUUAC9A section 5.2.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct DesignCapacity {
    pub milliamp_hours: u16,
}

super::register!(DesignCapacity, 0x3c, None);
