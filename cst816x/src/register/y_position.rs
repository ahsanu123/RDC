use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// Combined YposH/YposL, 0x05-0x06 (CST816S register declaration, page 1).
#[bitsize(16)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct YPosition {
    pub coordinate: u12,
    pub unspecified: u4,
}

impl YPosition {
    /// Address of the high byte of this combined register.
    pub const ADDRESS: Address = Address::YPositionHigh;
}

#[cfg(feature = "defmt")]
impl defmt::Format for YPosition {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "YPosition({=u16:#x})", u16::from(*self));
    }
}

impl Register {
    pub(crate) fn read_y_position<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<YPosition, Cst816sError<I::Error>> {
        let bytes = comproc::read_touch_window(i2c, device_address, YPosition::ADDRESS)?;
        Ok(u16::from_be_bytes(bytes).into())
    }
}
