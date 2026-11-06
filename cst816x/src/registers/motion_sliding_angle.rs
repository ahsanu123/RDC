use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// MotionSlAngle, 0xEF (CST816S register declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct MotionSlidingAngle {
    /// Raw tan(c) * 10 setting; the PDF does not specify angle units or rounding.
    pub tangent_times_ten: u8,
}

impl MotionSlidingAngle {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::MotionSlidingAngle;
}

#[cfg(feature = "defmt")]
impl defmt::Format for MotionSlidingAngle {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "MotionSlidingAngle({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_motion_sliding_angle<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<MotionSlidingAngle, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, MotionSlidingAngle::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_motion_sliding_angle<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: MotionSlidingAngle,
    ) -> Result<(), Cst816sError<I::Error>> {
        comproc::write(
            i2c,
            device_address,
            MotionSlidingAngle::ADDRESS,
            value.into(),
        )
    }
}
