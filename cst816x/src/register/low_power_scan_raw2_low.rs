use bilge::prelude::*;

use super::Address;

/// LpScanRaw2L, 0xF3: channel 2 reference low byte (CST816S declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanRaw2Low {
    pub low_byte: u8,
}

impl LowPowerScanRaw2Low {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::LowPowerScanRaw2Low;
}

#[cfg(feature = "defmt")]
impl defmt::Format for LowPowerScanRaw2Low {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "LowPowerScanRaw2Low({=u8:#x})", u8::from(*self));
    }
}
