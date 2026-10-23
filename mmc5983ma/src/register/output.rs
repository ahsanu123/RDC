use bilge::prelude::*;

/// One complete output byte: X/Y/Z high/low, temperature or product ID
/// (Rev A, pages 13-16). The address determines its meaning.
#[bitsize(8)]
#[derive(FromBits, DebugBits, Default, Clone, Copy, PartialEq, Eq)]
pub struct OutputByte {
    pub value: u8,
}

/// XYZout2: reserved bits 1:0, Z 3:2, Y 5:4, X 7:6 (Rev A, page 13).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Default, Clone, Copy, PartialEq, Eq)]
pub struct OutputLeastSignificantBits {
    pub reserved: u2,
    pub z: u2,
    pub y: u2,
    pub x: u2,
}

/// Seven sequential magnetic output registers (Rev A, pages 13-14).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MagneticOutputRegisters {
    pub x_high: OutputByte,
    pub x_low: OutputByte,
    pub y_high: OutputByte,
    pub y_low: OutputByte,
    pub z_high: OutputByte,
    pub z_low: OutputByte,
    pub least_significant_bits: OutputLeastSignificantBits,
}

impl From<[u8; 7]> for MagneticOutputRegisters {
    fn from(bytes: [u8; 7]) -> Self {
        Self {
            x_high: bytes[0].into(),
            x_low: bytes[1].into(),
            y_high: bytes[2].into(),
            y_low: bytes[3].into(),
            z_high: bytes[4].into(),
            z_low: bytes[5].into(),
            least_significant_bits: bytes[6].into(),
        }
    }
}
