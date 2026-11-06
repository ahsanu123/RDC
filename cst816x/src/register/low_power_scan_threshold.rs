use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// LpScanTH, 0xF5 (CST816S register declaration, page 4).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanThreshold {
    /// Wake-up threshold; range: 1-255. Smaller values increase sensitivity.
    pub threshold: u8,
}

impl Default for LowPowerScanThreshold {
    fn default() -> Self {
        Self::new(48)
    }
}

impl LowPowerScanThreshold {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::LowPowerScanThreshold;
}

#[cfg(feature = "defmt")]
impl defmt::Format for LowPowerScanThreshold {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(
            formatter,
            "LowPowerScanThreshold({=u8:#x})",
            u8::from(*self)
        );
    }
}

impl Register {
    pub(crate) fn read_low_power_scan_threshold<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<LowPowerScanThreshold, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, LowPowerScanThreshold::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_low_power_scan_threshold<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: LowPowerScanThreshold,
    ) -> Result<(), Cst816sError<I::Error>> {
        comproc::write(
            i2c,
            device_address,
            LowPowerScanThreshold::ADDRESS,
            value.into(),
        )
    }
}
