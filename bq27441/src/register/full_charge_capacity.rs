use bilge::prelude::*;

/// FullChargeCapacity() in mAh; SLUUAC9A section 4.8.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct FullChargeCapacity {
    pub milliamp_hours: u16,
}

super::register!(FullChargeCapacity, 0x0e, None);
