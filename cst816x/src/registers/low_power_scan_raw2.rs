use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// Combined LpScanRaw2H/LpScanRaw2L, 0xF2-0xF3 (CST816S declaration, page 3).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanRaw2 {
    pub value: u16,
}

impl LowPowerScanRaw2 {
    /// Address of the high byte of this combined register.
    pub const ADDRESS: Address = Address::LowPowerScanRaw2High;
}

#[cfg(feature = "defmt")]
impl defmt::Format for LowPowerScanRaw2 {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "LowPowerScanRaw2({=u16:#x})", u16::from(*self));
    }
}

impl Register {
    pub(crate) fn read_low_power_scan_raw2<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<LowPowerScanRaw2, Cst816sError<I::Error>> {
        let high = comproc::read(i2c, device_address, Address::LowPowerScanRaw2High)?;
        let low = comproc::read(i2c, device_address, Address::LowPowerScanRaw2Low)?;
        Ok(u16::from_be_bytes([high, low]).into())
    }

    pub(crate) fn write_low_power_scan_raw2<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: LowPowerScanRaw2,
    ) -> Result<(), Cst816sError<I::Error>> {
        let [high, low] = u16::from(value).to_be_bytes();
        comproc::write(i2c, device_address, Address::LowPowerScanRaw2High, high)?;
        comproc::write(i2c, device_address, Address::LowPowerScanRaw2Low, low)
    }
}
