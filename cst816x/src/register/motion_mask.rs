use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// MotionMask, 0xEC (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct MotionMask {
    pub double_click_enabled: bool,
    pub continuous_up_down_enabled: bool,
    pub continuous_left_right_enabled: bool,
    pub unspecified: u5,
}

impl MotionMask {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::MotionMask;
}

#[cfg(feature = "defmt")]
impl defmt::Format for MotionMask {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "MotionMask({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_motion_mask<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<MotionMask, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, MotionMask::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_motion_mask<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: MotionMask,
    ) -> Result<(), Cst816sError<I::Error>> {
        let current = comproc::read(i2c, device_address, MotionMask::ADDRESS)?;
        let updated = (current & !0x07) | (u8::from(value) & 0x07);
        comproc::write(i2c, device_address, MotionMask::ADDRESS, updated)
    }
}
