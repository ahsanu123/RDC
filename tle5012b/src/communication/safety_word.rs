use bilge::prelude::*;
#[bitsize(16)]
#[derive(FromBits)]
// LSB first
pub struct SafetyWord {
    // TODO: add typed bit field
    chip_status: u4,
    response: u4,
    crc: u8,
}
