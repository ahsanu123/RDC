use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// LpScanIdac, 0xF8 (CST816S register declaration, page 4).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanCurrent {
    /// Raw current setting: 1-255; smaller values increase sensitivity.
    pub current: u8,
}

impl LowPowerScanCurrent {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::LowPowerScanCurrent;
}

#[cfg(feature = "defmt")]
impl defmt::Format for LowPowerScanCurrent {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "LowPowerScanCurrent({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_low_power_scan_current<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<LowPowerScanCurrent, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, LowPowerScanCurrent::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_low_power_scan_current<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: LowPowerScanCurrent,
    ) -> Result<(), Cst816sError<I::Error>> {
        comproc::write(
            i2c,
            device_address,
            LowPowerScanCurrent::ADDRESS,
            value.into(),
        )
    }
}
