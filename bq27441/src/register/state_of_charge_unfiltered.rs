use bilge::prelude::*;

/// StateOfChargeUnfiltered() in percent; SLUUAC9A section 4.20.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct StateOfChargeUnfiltered {
    pub percent: u16,
}

super::register!(StateOfChargeUnfiltered, 0x30, None);
