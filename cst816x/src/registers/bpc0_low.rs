use bilge::prelude::*;

use super::Address;

/// BPC0L, 0xB1: `BPC0[7:0]` (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct Bpc0Low {
    pub low_byte: u8,
}

impl Bpc0Low {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::Bpc0Low;
}

#[cfg(feature = "defmt")]
impl defmt::Format for Bpc0Low {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "Bpc0Low({=u8:#x})", u8::from(*self));
    }
}
