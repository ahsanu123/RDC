use bilge::prelude::*;

use super::Address;

/// YposH, 0x05: `Y[11:8]` (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct YPositionHigh {
    pub coordinate: u4,
    pub unspecified: u4,
}

impl YPositionHigh {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::YPositionHigh;
}

#[cfg(feature = "defmt")]
impl defmt::Format for YPositionHigh {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "YPositionHigh({=u8:#x})", u8::from(*self));
    }
}
