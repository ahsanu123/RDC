use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// LpScanWin, 0xF6 (CST816S register declaration, page 4).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerScanWindow {
    /// Raw measurement range: 0-3. Larger values increase sensitivity and power.
    pub window: u8,
}

impl Default for LowPowerScanWindow {
    fn default() -> Self {
        Self::new(3)
    }
}

impl LowPowerScanWindow {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::LowPowerScanWindow;
}

#[cfg(feature = "defmt")]
impl defmt::Format for LowPowerScanWindow {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "LowPowerScanWindow({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_low_power_scan_window<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<LowPowerScanWindow, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, LowPowerScanWindow::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_low_power_scan_window<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: LowPowerScanWindow,
    ) -> Result<(), Cst816sError<I::Error>> {
        comproc::write(
            i2c,
            device_address,
            LowPowerScanWindow::ADDRESS,
            value.into(),
        )
    }
}
