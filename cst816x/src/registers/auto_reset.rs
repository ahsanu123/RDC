use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// AutoReset / DebounceTime, 0xFB (CST816S register declaration, page 5).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct AutoReset {
    /// Reset after touch without a valid gesture for this time; 0 disables.
    pub seconds: u8,
}

impl AutoReset {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::AutoReset;
}

#[cfg(feature = "defmt")]
impl defmt::Format for AutoReset {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "AutoReset({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_auto_reset<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<AutoReset, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, AutoReset::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_auto_reset<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: AutoReset,
    ) -> Result<(), Cst816sError<I::Error>> {
        comproc::write(i2c, device_address, AutoReset::ADDRESS, value.into())
    }
}
