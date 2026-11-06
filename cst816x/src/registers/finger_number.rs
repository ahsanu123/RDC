use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// FingerNum codes (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(TryFromBits, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FingerCount {
    NoFinger = 0,
    OneFinger = 1,
}

/// FingerNum, 0x02 (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(TryFromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct FingerNumber {
    pub count: FingerCount,
}

impl FingerNumber {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::FingerNumber;
}

#[cfg(feature = "defmt")]
impl defmt::Format for FingerNumber {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "FingerNumber({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_finger_number<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<FingerNumber, Cst816sError<I::Error>> {
        let value = comproc::read(i2c, device_address, FingerNumber::ADDRESS)?;
        FingerNumber::try_from(value).map_err(|_| Cst816sError::InvalidRegisterValue {
            address: FingerNumber::ADDRESS,
            value,
        })
    }
}
