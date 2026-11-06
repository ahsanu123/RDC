use bilge::prelude::*;

use super::Address;

/// XposH, 0x03: `X[11:8]` (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct XPositionHigh {
    pub coordinate: u4,
    pub unspecified: u4,
}

impl XPositionHigh {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::XPositionHigh;
}

#[cfg(feature = "defmt")]
impl defmt::Format for XPositionHigh {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "XPositionHigh({=u8:#x})", u8::from(*self));
    }
}
