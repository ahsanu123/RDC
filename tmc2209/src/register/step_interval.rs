use bilge::prelude::*;

/// TSTEP, 1/256-step clock interval; all ones means overflow/standstill. Rev. 1.09, section 5.2.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct StepInterval {
    pub clock_cycles: u20,
    pub unused: u12,
}

super::register!(StepInterval, 0x12, None);
impl super::ReadableRegister for StepInterval {}
