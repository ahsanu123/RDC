use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// Combined XposH/XposL, 0x03-0x04 (CST816S register declaration, page 1).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct XPosition {
    pub coordinate: u12,
    pub unspecified: u4,
}

impl XPosition {
    /// Address of the high byte of this combined register.
    pub const ADDRESS: Address = Address::XPositionHigh;
}

#[cfg(feature = "defmt")]
impl defmt::Format for XPosition {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "XPosition({=u16:#x})", u16::from(*self));
    }
}

impl Register {
    pub(crate) fn read_x_position<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<XPosition, Cst816sError<I::Error>> {
        let high = comproc::read(i2c, device_address, Address::XPositionHigh)?;
        let low = comproc::read(i2c, device_address, Address::XPositionLow)?;
        Ok(u16::from_be_bytes([high, low]).into())
    }
}
