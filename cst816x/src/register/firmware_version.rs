use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// FwVersion, 0xA9 (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct FirmwareVersion {
    pub version: u8,
}

impl FirmwareVersion {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::FirmwareVersion;
}

#[cfg(feature = "defmt")]
impl defmt::Format for FirmwareVersion {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "FirmwareVersion({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_firmware_version<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<FirmwareVersion, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, FirmwareVersion::ADDRESS).map(Into::into)
    }
}
