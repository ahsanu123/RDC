use bilge::prelude::*;

use super::Address;

/// LpScanRaw2H, 0xF2: channel 2 reference high byte (CST816S declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanRaw2High {
    pub high_byte: u8,
}

impl LowPowerScanRaw2High {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::LowPowerScanRaw2High;
}

#[cfg(feature = "defmt")]
impl defmt::Format for LowPowerScanRaw2High {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "LowPowerScanRaw2High({=u8:#x})", u8::from(*self));
    }
}
