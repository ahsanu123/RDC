use bilge::prelude::*;

/// Voltage() in mV; SLUUAC9A section 4.3.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Voltage {
    pub millivolts: u16,
}

super::register!(Voltage, 0x04, None);
