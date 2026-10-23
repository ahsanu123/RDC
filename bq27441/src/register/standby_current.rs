use bilge::prelude::*;

/// StandbyCurrent() signed two's-complement mA; SLUUAC9A section 4.10.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct StandbyCurrent {
    pub raw: u16,
}

super::register!(StandbyCurrent, 0x12, None);
