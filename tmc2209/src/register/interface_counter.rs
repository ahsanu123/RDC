use bilge::prelude::*;

/// IFCNT, successful write count modulo 256. Rev. 1.09, section 5.1, page 24.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct InterfaceCounter {
    pub count: u8,
    pub unused: u24,
}

super::register!(InterfaceCounter, 0x02, Some(0));
impl super::ReadableRegister for InterfaceCounter {}
