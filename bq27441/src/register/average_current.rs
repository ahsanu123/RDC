use bilge::prelude::*;

/// AverageCurrent() signed two's-complement mA; SLUUAC9A section 4.9.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct AverageCurrent {
    pub raw: u16,
}

super::register!(AverageCurrent, 0x10, None);
