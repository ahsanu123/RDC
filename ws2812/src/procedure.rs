use crate::{Error, RgbColor, function::ColorEncoding, validation};

/// Nominal clock; the actual peripheral clock may be 3.0-3.2 MHz.
pub const SPI_FREQUENCY_HZ: u32 = 3_200_000;
/// Four SPI bits for each of the 24 GRB bits.
pub const ENCODED_BYTES_PER_PIXEL: usize = 12;
/// 320 us of low data at the maximum supported clock, strictly above 280 us.
pub const RESET_BYTES: usize = 128;

pub(crate) const ZERO_SYMBOL: u8 = 0b1000;
pub(crate) const ONE_SYMBOL: u8 = 0b1100;
pub(crate) static RESET_DATA: [u8; RESET_BYTES] = [0; RESET_BYTES];

/// Compatibility representation of a rounded-up nominal duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Nanos {
    pub value: u32,
}

/// Nominal 3.2 MHz timings, rounded up to whole nanoseconds.
///
/// Compatibility names from the original driver; encoding uses SPI ticks,
/// not software delays. WS2812B-2020 timing reference: datasheet page 4.
pub struct Constant {}

impl Constant {
    pub const T0H: Nanos = Nanos { value: 313 };
    pub const T1H: Nanos = Nanos { value: 625 };
    pub const T0L: Nanos = Nanos { value: 938 };
    pub const T1L: Nanos = Nanos { value: 625 };
    pub const RES: Nanos = Nanos { value: 320_000 };
    pub const HIGH: u8 = ONE_SYMBOL;
    pub const LOW: u8 = ZERO_SYMBOL;
}

pub(crate) fn encode_frame<'buffer, E>(
    encoder: &ColorEncoding,
    colors: &[RgbColor],
    scratch: &'buffer mut [u8],
) -> Result<&'buffer [u8], Error<E>> {
    let required = validation::payload_length(colors.len(), scratch.len())?;
    let payload = &mut scratch[..required];
    for (color, bytes) in colors
        .iter()
        .zip(payload.as_chunks_mut::<ENCODED_BYTES_PER_PIXEL>().0)
    {
        bytes.copy_from_slice(&encoder.encode(*color));
    }
    Ok(payload)
}
