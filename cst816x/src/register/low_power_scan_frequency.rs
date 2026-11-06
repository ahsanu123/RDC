use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// LpScanFreq, 0xF7 (CST816S register declaration, page 4).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanFrequency {
    /// Raw frequency setting: 1-255; smaller values increase sensitivity.
    pub frequency: u8,
}

impl Default for LowPowerScanFrequency {
    fn default() -> Self {
        Self::new(7)
    }
}

impl LowPowerScanFrequency {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::LowPowerScanFrequency;
}

#[cfg(feature = "defmt")]
impl defmt::Format for LowPowerScanFrequency {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(
            formatter,
            "LowPowerScanFrequency({=u8:#x})",
            u8::from(*self)
        );
    }
}

impl Register {
    pub(crate) fn read_low_power_scan_frequency<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<LowPowerScanFrequency, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, LowPowerScanFrequency::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_low_power_scan_frequency<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: LowPowerScanFrequency,
    ) -> Result<(), Cst816sError<I::Error>> {
        comproc::write(
            i2c,
            device_address,
            LowPowerScanFrequency::ADDRESS,
            value.into(),
        )
    }
}
