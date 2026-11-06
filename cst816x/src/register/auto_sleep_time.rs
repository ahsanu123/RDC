use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// AutoSleepTime, 0xF9 (CST816S register declaration, page 4).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct AutoSleepTime {
    /// Time without touch before entering low-power mode.
    pub seconds: u8,
}

impl Default for AutoSleepTime {
    fn default() -> Self {
        Self::new(2)
    }
}

impl AutoSleepTime {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::AutoSleepTime;
}

#[cfg(feature = "defmt")]
impl defmt::Format for AutoSleepTime {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "AutoSleepTime({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_auto_sleep_time<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<AutoSleepTime, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, AutoSleepTime::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_auto_sleep_time<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: AutoSleepTime,
    ) -> Result<(), Cst816sError<I::Error>> {
        comproc::write(i2c, device_address, AutoSleepTime::ADDRESS, value.into())
    }
}
