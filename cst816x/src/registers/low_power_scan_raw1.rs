use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// Combined LpScanRaw1H/LpScanRaw1L, 0xF0-0xF1 (CST816S declaration, page 3).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanRaw1 {
    pub value: u16,
}

impl LowPowerScanRaw1 {
    /// Address of the high byte of this combined register.
    pub const ADDRESS: Address = Address::LowPowerScanRaw1High;
}

#[cfg(feature = "defmt")]
impl defmt::Format for LowPowerScanRaw1 {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "LowPowerScanRaw1({=u16:#x})", u16::from(*self));
    }
}

impl Register {
    pub(crate) fn read_low_power_scan_raw1<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<LowPowerScanRaw1, Cst816sError<I::Error>> {
        let high = comproc::read(i2c, device_address, Address::LowPowerScanRaw1High)?;
        let low = comproc::read(i2c, device_address, Address::LowPowerScanRaw1Low)?;
        Ok(u16::from_be_bytes([high, low]).into())
    }

    pub(crate) fn write_low_power_scan_raw1<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: LowPowerScanRaw1,
    ) -> Result<(), Cst816sError<I::Error>> {
        let [high, low] = u16::from(value).to_be_bytes();
        comproc::write(i2c, device_address, Address::LowPowerScanRaw1High, high)?;
        comproc::write(i2c, device_address, Address::LowPowerScanRaw1Low, low)
    }
}
