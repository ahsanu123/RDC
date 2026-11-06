use bilge::prelude::*;

use super::Address;

/// BPC1L, 0xB3: `BPC1[7:0]` (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct Bpc1Low {
    pub low_byte: u8,
}

impl Bpc1Low {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::Bpc1Low;
}

#[cfg(feature = "defmt")]
impl defmt::Format for Bpc1Low {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "Bpc1Low({=u8:#x})", u8::from(*self));
    }
}
