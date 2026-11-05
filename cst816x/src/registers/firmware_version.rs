use bilge::prelude::*;

/// FwVersion, 0xA9 (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct FirmwareVersion {
    pub version: u8,
}
