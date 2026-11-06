use crate::register::Address;

/// Invalid seven-bit destination, retaining the untouched owned bus.
#[derive(Debug)]
pub struct Cst816sAddressError<I> {
    /// Rejected unshifted address.
    pub address: u8,
    /// Bus supplied to the constructor; no transfer has occurred.
    pub i2c: I,
}

#[cfg(feature = "defmt")]
impl<I> defmt::Format for Cst816sAddressError<I> {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "InvalidI2cAddress({=u8:#x})", self.address);
    }
}

/// Device errors retain the original I2C error or the invalid register byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cst816sError<InterfaceError> {
    Interface(InterfaceError),
    InvalidRegisterValue { address: Address, value: u8 },
}
