use bilge::prelude::*;

/// MSCURACT, signed 9-bit phase table values before current scaling. Rev. 1.09, section 5.4.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct MicrostepCurrent {
    pub phase_b_bits: u9,
    pub unused_low: u7,
    pub phase_a_bits: u9,
    pub unused_high: u7,
}

super::register!(MicrostepCurrent, 0x6b, None);
impl super::ReadableRegister for MicrostepCurrent {}
