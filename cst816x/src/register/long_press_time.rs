use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// LongPressTime, 0xFC (CST816S register declaration, page 5).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct LongPressTime {
    /// Reset after a long press for this time; 0 disables.
    pub seconds: u8,
}

impl Default for LongPressTime {
    fn default() -> Self {
        Self::new(10)
    }
}

impl LongPressTime {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::LongPressTime;
}

#[cfg(feature = "defmt")]
impl defmt::Format for LongPressTime {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "LongPressTime({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_long_press_time<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<LongPressTime, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, LongPressTime::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_long_press_time<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: LongPressTime,
    ) -> Result<(), Cst816sError<I::Error>> {
        comproc::write(i2c, device_address, LongPressTime::ADDRESS, value.into())
    }
}
