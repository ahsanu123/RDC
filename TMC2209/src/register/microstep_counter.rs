use bilge::prelude::*;

/// MSCNT, position in the 1024-entry electrical wave table. Rev. 1.09, section 5.4.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct MicrostepCounter {
    pub position: u10,
    pub unused: u22,
}

super::register!(MicrostepCounter, 0x6a, None);
impl super::ReadableRegister for MicrostepCounter {}
