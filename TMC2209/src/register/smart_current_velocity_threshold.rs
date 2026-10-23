use bilge::prelude::*;

/// TCOOLTHRS, lower CoolStep/StallGuard velocity threshold. Rev. 1.09, section 5.3, page 29.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct SmartCurrentVelocityThreshold {
    pub clock_cycles: u20,
    pub unused: u12,
}

super::register!(SmartCurrentVelocityThreshold, 0x14, Some(0));
impl super::WritableRegister for SmartCurrentVelocityThreshold {
    const WRITE_MASK: u32 = 0x000f_ffff;
}
