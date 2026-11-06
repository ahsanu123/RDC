use bilge::prelude::*;

use super::Address;

/// YposL, 0x06: `Y[7:0]` (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct YPositionLow {
    pub coordinate: u8,
}

impl YPositionLow {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::YPositionLow;
}

#[cfg(feature = "defmt")]
impl defmt::Format for YPositionLow {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "YPositionLow({=u8:#x})", u8::from(*self));
    }
}
