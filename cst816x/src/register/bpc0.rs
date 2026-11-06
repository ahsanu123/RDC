use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// Combined BPC0H/BPC0L, 0xB0-0xB1 (CST816S register declaration, pages 1-2).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct Bpc0 {
    pub value: u16,
}

impl Bpc0 {
    /// Address of the high byte of this combined register.
    pub const ADDRESS: Address = Address::Bpc0High;
}

#[cfg(feature = "defmt")]
impl defmt::Format for Bpc0 {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "Bpc0({=u16:#x})", u16::from(*self));
    }
}

impl Register {
    pub(crate) fn read_bpc0<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<Bpc0, Cst816sError<I::Error>> {
        let high = comproc::read(i2c, device_address, Address::Bpc0High)?;
        let low = comproc::read(i2c, device_address, Address::Bpc0Low)?;
        Ok(u16::from_be_bytes([high, low]).into())
    }
}
