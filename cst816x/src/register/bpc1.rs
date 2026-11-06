use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// Combined BPC1H/BPC1L, 0xB2-0xB3 (CST816S register declaration, page 2).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct Bpc1 {
    pub value: u16,
}

impl Bpc1 {
    /// Address of the high byte of this combined register.
    pub const ADDRESS: Address = Address::Bpc1High;
}

#[cfg(feature = "defmt")]
impl defmt::Format for Bpc1 {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "Bpc1({=u16:#x})", u16::from(*self));
    }
}

impl Register {
    pub(crate) fn read_bpc1<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<Bpc1, Cst816sError<I::Error>> {
        let high = comproc::read(i2c, device_address, Address::Bpc1High)?;
        let low = comproc::read(i2c, device_address, Address::Bpc1Low)?;
        Ok(u16::from_be_bytes([high, low]).into())
    }
}
