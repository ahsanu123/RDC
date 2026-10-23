use bilge::prelude::*;

/// TABLE access control, datasheet Rev. 7, page 14; byte writes required.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct ModelAccess {
    pub low_key: u8,
    pub high_key: u8,
}

impl ModelAccess {
    /// Address of the low key byte, written first.
    pub const LOW_ADDRESS: u8 = 0x3f;
    /// Address of the high key byte, written second.
    pub const HIGH_ADDRESS: u8 = 0x3e;
    /// Manufacturer's TABLE unlock key.
    pub const UNLOCK_KEY: u16 = 0x4a57;
    /// Value written to each key byte when relocking.
    pub const LOCK_BYTE: u8 = 0;
}

/// One opaque TABLE word, datasheet Rev. 7, page 14 (32 words total).
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct ModelTableWord {
    pub value: u16,
}

impl ModelTableWord {
    /// First TABLE word address.
    pub const START_ADDRESS: u8 = 0x40;
    /// Number of words in the 64-byte model.
    pub const COUNT: usize = 32;
}
