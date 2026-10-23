use bilge::prelude::*;

/// GSTAT, reset/error flags; write one to clear. Rev. 1.09, section 5.1, page 24.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct GlobalStatus {
    pub reset_detected: bool,
    pub driver_error: bool,
    pub charge_pump_undervoltage: bool,
    pub unused: u29,
}

super::register!(GlobalStatus, 0x01, None);
impl super::ReadableRegister for GlobalStatus {}
impl super::WritableRegister for GlobalStatus {
    const WRITE_MASK: u32 = 0x0000_0003;
}
