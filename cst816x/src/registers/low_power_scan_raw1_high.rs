use bilge::prelude::*;

use super::Address;

/// LpScanRaw1H, 0xF0: channel 1 reference high byte (CST816S declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanRaw1High {
    pub high_byte: u8,
}

impl LowPowerScanRaw1High {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::LowPowerScanRaw1High;
}

#[cfg(feature = "defmt")]
impl defmt::Format for LowPowerScanRaw1High {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "LowPowerScanRaw1High({=u8:#x})", u8::from(*self));
    }
}
