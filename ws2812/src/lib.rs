#![no_std]
#![doc = include_str!("../README.md")]

mod comproc;
mod error;
mod function;
mod procedure;
mod validation;

use embedded_hal::spi::{ErrorType, SpiDevice};
use function::ColorEncoding;

pub use error::Error;
pub use procedure::{Constant, ENCODED_BYTES_PER_PIXEL, Nanos, RESET_BYTES, SPI_FREQUENCY_HZ};
pub use validation::{MAX_SPI_FREQUENCY_HZ, MIN_SPI_FREQUENCY_HZ};

/// Eight-bit color channels, transmitted as GRB (WS2812B-2020, page 5).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// Compatibility name for the original RGB type.
pub use RgbColor as RGB;

/// Worldsemi WS2812B-2020 SPI waveform driver (datasheet pages 4-5).
///
/// SPI must be configured externally for MSB-first, mode 0, 3.0-3.2 MHz.
/// The physical MOSI line must be dedicated to the LEDs: they have no CS input.
pub struct Ws2812<SPI> {
    spi: SPI,
    color_encoding: ColorEncoding,
}

impl<SPI: ErrorType> Ws2812<SPI> {
    /// Take ownership without I/O; assumes externally configured 3.2 MHz SPI.
    ///
    /// Use try_new to validate a caller-reported clock. Neither constructor
    /// configures or measures the SPI clock.
    pub fn new(spi: SPI) -> Self {
        Self {
            spi,
            color_encoding: ColorEncoding,
        }
    }

    /// Validate the reported actual SPI clock before taking ownership.
    ///
    /// On failure, the passed SPI device is dropped without I/O.
    pub fn try_new(spi: SPI, spi_frequency_hz: u32) -> Result<Self, Error<SPI::Error>> {
        validation::validate_spi_frequency(spi_frequency_hz)?;
        Ok(Self::new(spi))
    }

    /// Return the SPI device without issuing a reset or changing the LEDs.
    pub fn release(self) -> SPI {
        self.spi
    }
}

impl<SPI: SpiDevice<u8>> Ws2812<SPI> {
    /// Send a complete array of pixels in one transaction, including reset/latch.
    ///
    /// Uses 12 * COLOR_COUNTS bytes of local stack storage. An empty array is
    /// a no-op. Use write_colors_buffered for a large chain.
    pub fn write_colors<const COLOR_COUNTS: usize>(
        &mut self,
        colors: &[RgbColor; COLOR_COUNTS],
    ) -> Result<(), Error<SPI::Error>> {
        if colors.is_empty() {
            return Ok(());
        }
        let encoded = colors.map(|color| self.color_encoding.encode(color));
        comproc::write_frame(&mut self.spi, encoded.as_flattened())
    }

    /// Send one complete frame for the first LED, not an append operation.
    pub fn write_color(&mut self, color: &RgbColor) -> Result<(), Error<SPI::Error>> {
        self.write_colors(&[*color])
    }

    /// Encode a color slice into caller storage and send one complete frame.
    ///
    /// At least 12 bytes per pixel are required. Unused scratch bytes are
    /// unchanged and not transmitted. Invalid input causes no bus access.
    pub fn write_colors_buffered(
        &mut self,
        colors: &[RgbColor],
        scratch: &mut [u8],
    ) -> Result<(), Error<SPI::Error>> {
        let encoded = procedure::encode_frame(&self.color_encoding, colors, scratch)?;
        if encoded.is_empty() {
            return Ok(());
        }
        comproc::write_frame(&mut self.spi, encoded)
    }

    /// Send a low reset/latch interval, which may latch previously partial data.
    ///
    /// This does not clear LEDs to black. A nonempty frame includes this
    /// operation automatically before and after the payload.
    pub fn latch(&mut self) -> Result<(), Error<SPI::Error>> {
        comproc::latch(&mut self.spi)
    }
}

#[cfg(feature = "async")]
impl<SPI: embedded_hal_async::spi::SpiDevice<u8>> Ws2812<SPI> {
    /// Async write_colors; cancellation may leave a partial frame on the chain.
    ///
    /// Stores 12 * COLOR_COUNTS payload bytes in the future. Timing within the
    /// payload must be guaranteed by the HAL, not by executor scheduling.
    pub async fn write_colors_async<const COLOR_COUNTS: usize>(
        &mut self,
        colors: &[RgbColor; COLOR_COUNTS],
    ) -> Result<(), Error<SPI::Error>> {
        if colors.is_empty() {
            return Ok(());
        }
        let encoded = colors.map(|color| self.color_encoding.encode(color));
        comproc::write_frame_async(&mut self.spi, encoded.as_flattened()).await
    }

    /// Async single-pixel frame; this does not append to an earlier frame.
    pub async fn write_color_async(&mut self, color: &RgbColor) -> Result<(), Error<SPI::Error>> {
        self.write_colors_async(&[*color]).await
    }

    /// Async complete frame using caller-provided payload storage.
    pub async fn write_colors_buffered_async(
        &mut self,
        colors: &[RgbColor],
        scratch: &mut [u8],
    ) -> Result<(), Error<SPI::Error>> {
        let encoded = procedure::encode_frame(&self.color_encoding, colors, scratch)?;
        if encoded.is_empty() {
            return Ok(());
        }
        comproc::write_frame_async(&mut self.spi, encoded).await
    }

    /// Async low reset/latch interval; this does not clear the chain.
    pub async fn latch_async(&mut self) -> Result<(), Error<SPI::Error>> {
        comproc::latch_async(&mut self.spi).await
    }
}
