use bilge::prelude::*;

/// Temperature() in 0.1 K; SLUUAC9A section 4.2.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Temperature {
    pub decikelvin: u16,
}

super::register!(Temperature, 0x02, None);
