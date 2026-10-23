use bilge::prelude::*;

/// TABLE access control, datasheet Rev. 7, page 14; byte writes required.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct ModelAccess {
    pub low_key: u8,
    pub high_key: u8,
}

/// One opaque TABLE word, datasheet Rev. 7, page 14 (32 words total).
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct ModelTableWord {
    pub value: u16,
}
