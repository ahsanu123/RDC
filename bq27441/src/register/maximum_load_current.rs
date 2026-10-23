use bilge::prelude::*;

/// MaxLoadCurrent() signed two's-complement mA; SLUUAC9A section 4.11.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct MaximumLoadCurrent {
    pub raw: u16,
}

super::register!(MaximumLoadCurrent, 0x14, None);
