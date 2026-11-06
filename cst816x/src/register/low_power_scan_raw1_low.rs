use bilge::prelude::*;

use super::Address;

/// LpScanRaw1L, 0xF1: channel 1 reference low byte (CST816S declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanRaw1Low {
    pub low_byte: u8,
}

impl LowPowerScanRaw1Low {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::LowPowerScanRaw1Low;
}

#[cfg(feature = "defmt")]
impl defmt::Format for LowPowerScanRaw1Low {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "LowPowerScanRaw1Low({=u8:#x})", u8::from(*self));
    }
}
