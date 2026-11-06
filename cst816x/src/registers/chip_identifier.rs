use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// ChipID, 0xA7 (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct ChipIdentifier {
    pub identifier: u8,
}

impl ChipIdentifier {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::ChipIdentifier;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ChipIdentifier {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "ChipIdentifier({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_chip_identifier<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<ChipIdentifier, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, ChipIdentifier::ADDRESS).map(Into::into)
    }
}
