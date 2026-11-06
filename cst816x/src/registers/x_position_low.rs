use bilge::prelude::*;

use super::Address;

/// XposL, 0x04: `X[7:0]` (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct XPositionLow {
    pub coordinate: u8,
}

impl XPositionLow {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::XPositionLow;
}

#[cfg(feature = "defmt")]
impl defmt::Format for XPositionLow {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "XPositionLow({=u8:#x})", u8::from(*self));
    }
}
