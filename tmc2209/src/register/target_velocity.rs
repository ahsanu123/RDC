use bilge::prelude::*;

/// VACTUAL, signed 24-bit two's-complement velocity; zero selects STEP/DIR. Rev. 1.09, sections 5.2, 14.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct TargetVelocity {
    pub signed_velocity_bits: u24,
    pub unused: u8,
}

super::register!(TargetVelocity, 0x22, Some(0));
impl super::WritableRegister for TargetVelocity {
    const WRITE_MASK: u32 = 0x00ff_ffff;
}
