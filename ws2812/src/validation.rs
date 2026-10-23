use crate::Error;

/// Lower bound imposed by the zero symbol's maximum 1 us low time.
pub const MIN_SPI_FREQUENCY_HZ: u32 = 3_000_000;
/// Upper bound imposed by the minimum 1.25 us total bit period.
pub const MAX_SPI_FREQUENCY_HZ: u32 = 3_200_000;

pub(crate) fn validate_spi_frequency<E>(frequency_hz: u32) -> Result<(), Error<E>> {
    if !(MIN_SPI_FREQUENCY_HZ..=MAX_SPI_FREQUENCY_HZ).contains(&frequency_hz) {
        return Err(Error::InvalidSpiFrequency {
            actual_hz: frequency_hz,
            minimum_hz: MIN_SPI_FREQUENCY_HZ,
            maximum_hz: MAX_SPI_FREQUENCY_HZ,
        });
    }
    Ok(())
}

pub(crate) fn payload_length<E>(color_count: usize, provided: usize) -> Result<usize, Error<E>> {
    let required = color_count
        .checked_mul(crate::ENCODED_BYTES_PER_PIXEL)
        .ok_or(Error::FrameTooLarge)?;
    if provided < required {
        return Err(Error::BufferTooSmall { required, provided });
    }
    Ok(required)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_length_rejects_overflow_before_buffer_checks() {
        assert_eq!(
            payload_length::<core::convert::Infallible>(usize::MAX / 12 + 1, usize::MAX),
            Err(Error::FrameTooLarge)
        );
    }
}
