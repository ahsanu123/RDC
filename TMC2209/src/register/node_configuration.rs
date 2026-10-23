use bilge::prelude::*;

/// NODECONF (formerly SLAVECONF), reply delay. Rev. 1.09, section 5.1, page 24.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct NodeConfiguration {
    pub unused_low: u8,
    pub reply_delay: u4,
    pub unused_high: u20,
}

super::register!(NodeConfiguration, 0x03, Some(0));
impl super::WritableRegister for NodeConfiguration {
    const WRITE_MASK: u32 = 0x0000_0f00;
}
