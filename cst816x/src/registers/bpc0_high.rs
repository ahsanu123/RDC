use bilge::prelude::*;

use super::Address;

/// BPC0H, 0xB0: `BPC0[15:8]` (CST816S register declaration, pages 1-2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct Bpc0High {
    pub high_byte: u8,
}

impl Bpc0High {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::Bpc0High;
}

#[cfg(feature = "defmt")]
impl defmt::Format for Bpc0High {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "Bpc0High({=u8:#x})", u8::from(*self));
    }
}
