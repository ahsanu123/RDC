use embedded_hal::i2c::I2c;

use super::{FingerNumber, GestureIdentifier, Register, XPosition, YPosition};
use crate::{comproc, error::Cst816sError};

impl Register {
    pub(crate) fn read_touch<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<(GestureIdentifier, FingerNumber, XPosition, YPosition), Cst816sError<I::Error>>
    {
        let [gesture, fingers, x_high, x_low, y_high, y_low] =
            comproc::read_touch_window(i2c, device_address, GestureIdentifier::ADDRESS)?;
        let gesture = GestureIdentifier::try_from(gesture).map_err(|_| {
            Cst816sError::InvalidRegisterValue {
                address: GestureIdentifier::ADDRESS,
                value: gesture,
            }
        })?;
        let fingers =
            FingerNumber::try_from(fingers).map_err(|_| Cst816sError::InvalidRegisterValue {
                address: FingerNumber::ADDRESS,
                value: fingers,
            })?;
        Ok((
            gesture,
            fingers,
            u16::from_be_bytes([x_high, x_low]).into(),
            u16::from_be_bytes([y_high, y_low]).into(),
        ))
    }
}
