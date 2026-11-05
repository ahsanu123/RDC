use crate::registers::Address;

/// Device errors retain the original I2C error or the invalid register byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cst816sError<InterfaceError> {
    Interface(InterfaceError),
    InvalidRegisterValue { address: Address, value: u8 },
}
