use bilge::prelude::*;

/// TPWMTHRS, upper StealthChop velocity threshold (TSTEP units). Rev. 1.09, section 5.2.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct PwmVelocityThreshold {
    pub clock_cycles: u20,
    pub unused: u12,
}

super::register!(PwmVelocityThreshold, 0x13, None);
impl super::WritableRegister for PwmVelocityThreshold {
    const WRITE_MASK: u32 = 0x000f_ffff;
}
