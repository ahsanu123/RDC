use embedded_hal::spi::{Operation, SpiDevice};

use crate::{Error, procedure::RESET_DATA};

pub(crate) fn write_frame<S: SpiDevice<u8>>(
    spi: &mut S,
    encoded: &[u8],
) -> Result<(), Error<S::Error>> {
    #[cfg(feature = "defmt")]
    defmt::trace!("WS2812 frame: {} payload bytes", encoded.len());
    spi.transaction(&mut [
        Operation::Write(&RESET_DATA),
        Operation::Write(encoded),
        Operation::Write(&RESET_DATA),
    ])
    .map_err(Error::Interface)
}

pub(crate) fn latch<S: SpiDevice<u8>>(spi: &mut S) -> Result<(), Error<S::Error>> {
    spi.write(&RESET_DATA).map_err(Error::Interface)
}

#[cfg(feature = "async")]
pub(crate) async fn write_frame_async<S: embedded_hal_async::spi::SpiDevice<u8>>(
    spi: &mut S,
    encoded: &[u8],
) -> Result<(), Error<S::Error>> {
    #[cfg(feature = "defmt")]
    defmt::trace!("WS2812 async frame: {} payload bytes", encoded.len());
    spi.transaction(&mut [
        Operation::Write(&RESET_DATA),
        Operation::Write(encoded),
        Operation::Write(&RESET_DATA),
    ])
    .await
    .map_err(Error::Interface)
}

#[cfg(feature = "async")]
pub(crate) async fn latch_async<S: embedded_hal_async::spi::SpiDevice<u8>>(
    spi: &mut S,
) -> Result<(), Error<S::Error>> {
    spi.write(&RESET_DATA).await.map_err(Error::Interface)
}
