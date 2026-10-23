use bilge::prelude::*;

/// AveragePower() signed two's-complement mW; SLUUAC9A section 4.12.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct AveragePower {
    pub raw: u16,
}

super::register!(AveragePower, 0x18, None);
