use bilge::prelude::*;

/// FullChargeCapacityUnfiltered() in mAh; SLUUAC9A section 4.18.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct FullChargeCapacityUnfiltered {
    pub milliamp_hours: u16,
}

super::register!(FullChargeCapacityUnfiltered, 0x2c, None);
