use bilge::prelude::*;

/// BlockData() bytes at 0x40-0x5f; SLUUAC9A section 5.5.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockData {
    pub bytes: [BlockDataByte; 32],
}

/// One byte of BlockData(); SLUUAC9A section 5.5.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct BlockDataByte {
    pub value: u8,
}

impl From<[u8; 32]> for BlockData {
    fn from(bytes: [u8; 32]) -> Self {
        Self {
            bytes: bytes.map(BlockDataByte::from),
        }
    }
}

impl From<BlockData> for [u8; 32] {
    fn from(block: BlockData) -> Self {
        block.bytes.map(u8::from)
    }
}
