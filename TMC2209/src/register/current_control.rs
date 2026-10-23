use bilge::prelude::*;

/// IHOLD_IRUN, complete write-only current settings. Rev. 1.09, section 5.2, page 28.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct CurrentControl {
    pub hold_current: u5,
    pub unused_hold: u3,
    pub run_current: u5,
    pub unused_run: u3,
    pub hold_delay: u4,
    pub unused_high: u12,
}

super::register!(CurrentControl, 0x10, None);
impl super::WritableRegister for CurrentControl {
    const WRITE_MASK: u32 = 0x000f_1f1f;
}
