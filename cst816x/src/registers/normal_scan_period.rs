use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// NorScanPer, 0xEE (CST816S register declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct NormalScanPeriod {
    /// Raw 10 ms units; range: 1-30. Affects auto-wake and auto-sleep timing.
    pub tens_of_milliseconds: u8,
}

impl Default for NormalScanPeriod {
    fn default() -> Self {
        Self::new(1)
    }
}

impl NormalScanPeriod {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::NormalScanPeriod;
}

#[cfg(feature = "defmt")]
impl defmt::Format for NormalScanPeriod {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "NormalScanPeriod({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_normal_scan_period<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<NormalScanPeriod, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, NormalScanPeriod::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_normal_scan_period<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: NormalScanPeriod,
    ) -> Result<(), Cst816sError<I::Error>> {
        comproc::write(i2c, device_address, NormalScanPeriod::ADDRESS, value.into())
    }
}
