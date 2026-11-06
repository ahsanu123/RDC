use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// GestureID codes (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(TryFromBits, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gesture {
    NoGesture = 0x00,
    SlideUp = 0x01,
    SlideDown = 0x02,
    SlideLeft = 0x03,
    SlideRight = 0x04,
    SingleClick = 0x05,
    DoubleClick = 0x0B,
    LongPress = 0x0C,
}

/// GestureID, 0x01 (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(TryFromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct GestureIdentifier {
    pub gesture: Gesture,
}

impl GestureIdentifier {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::GestureIdentifier;
}

#[cfg(feature = "defmt")]
impl defmt::Format for GestureIdentifier {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "GestureIdentifier({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_gesture_identifier<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<GestureIdentifier, Cst816sError<I::Error>> {
        let value = comproc::read(i2c, device_address, GestureIdentifier::ADDRESS)?;
        GestureIdentifier::try_from(value).map_err(|_| Cst816sError::InvalidRegisterValue {
            address: GestureIdentifier::ADDRESS,
            value,
        })
    }
}
