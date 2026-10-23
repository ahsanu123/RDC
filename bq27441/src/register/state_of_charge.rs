use bilge::prelude::*;

/// StateOfCharge() in percent (0-100); SLUUAC9A section 4.13.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct StateOfCharge {
    pub percent: u16,
}

super::register!(StateOfCharge, 0x1c, None);
