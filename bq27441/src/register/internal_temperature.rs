use bilge::prelude::*;

/// InternalTemperature() in 0.1 K; SLUUAC9A section 4.14.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct InternalTemperature {
    pub decikelvin: u16,
}

super::register!(InternalTemperature, 0x1e, None);
