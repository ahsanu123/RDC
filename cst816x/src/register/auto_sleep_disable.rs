use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// DisAutoSleep, 0xFE (CST816S register declaration, page 5).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct AutoSleepDisable {
    /// Zero enables automatic sleep; any nonzero byte disables it.
    pub disable: u8,
}

impl Default for AutoSleepDisable {
    fn default() -> Self {
        Self::new(0)
    }
}

impl AutoSleepDisable {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::AutoSleepDisable;
}

#[cfg(feature = "defmt")]
impl defmt::Format for AutoSleepDisable {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "AutoSleepDisable({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_auto_sleep_disable<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<AutoSleepDisable, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, AutoSleepDisable::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_auto_sleep_disable<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: AutoSleepDisable,
    ) -> Result<(), Cst816sError<I::Error>> {
        comproc::write(i2c, device_address, AutoSleepDisable::ADDRESS, value.into())
    }
}
