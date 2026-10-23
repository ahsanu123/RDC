/// Device errors retain the original interface error without translation.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    /// I2C transaction failed, including the expected NACK on reset.
    I2C(E),
    /// An input is out of range, reversed, or not exactly representable.
    InvalidInputData,
    /// Model transfer or relocking failed. Every encountered error is retained.
    ModelTable {
        /// First unlock or table-data error, if any.
        operation: Option<E>,
        /// Failure writing zero to 0x3F, if any.
        relock_low: Option<E>,
        /// Failure writing zero to 0x3E, if any.
        relock_high: Option<E>,
    },
}
