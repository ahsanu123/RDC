use bilge::prelude::*;

use super::Address;

/// BPC1H, 0xB2: `BPC1[15:8]` (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct Bpc1High {
    pub high_byte: u8,
}

impl Bpc1High {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::Bpc1High;
}

#[cfg(feature = "defmt")]
impl defmt::Format for Bpc1High {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "Bpc1High({=u8:#x})", u8::from(*self));
    }
}
