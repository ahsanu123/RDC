use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// LpAutoWakeTime, 0xF4 (CST816S register declaration, pages 3-4).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LowPowerAutoWakeTime {
    /// Low-power automatic recalibration period; range: 1-5 minutes.
    pub minutes: u8,
}

impl Default for LowPowerAutoWakeTime {
    fn default() -> Self {
        Self::new(5)
    }
}

impl LowPowerAutoWakeTime {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::LowPowerAutoWakeTime;
}

#[cfg(feature = "defmt")]
impl defmt::Format for LowPowerAutoWakeTime {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "LowPowerAutoWakeTime({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_low_power_auto_wake_time<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<LowPowerAutoWakeTime, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, LowPowerAutoWakeTime::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_low_power_auto_wake_time<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: LowPowerAutoWakeTime,
    ) -> Result<(), Cst816sError<I::Error>> {
        comproc::write(
            i2c,
            device_address,
            LowPowerAutoWakeTime::ADDRESS,
            value.into(),
        )
    }
}
