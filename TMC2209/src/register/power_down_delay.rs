use bilge::prelude::*;

/// TPOWERDOWN, delay in units of 2^18 clock cycles. Rev. 1.09, section 5.2, page 28.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct PowerDownDelay {
    pub delay: u8,
    pub unused: u24,
}

super::register!(PowerDownDelay, 0x11, Some(20));
impl super::WritableRegister for PowerDownDelay {
    const WRITE_MASK: u32 = 0x0000_00ff;
}
