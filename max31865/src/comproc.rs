//! MAX31865 Rev 3, pages 16-18: SPI address and payload framing.

use embedded_hal::spi::{Operation, SpiDevice};

use crate::error::Max31865Err;

pub(crate) fn read<S: SpiDevice<u8>>(
    spi: &mut S,
    address: u8,
    bytes: &mut [u8],
) -> Result<(), Max31865Err<S::Error>> {
    let command = [address & 0x7f];
    spi.transaction(&mut [Operation::Write(&command), Operation::Read(bytes)])
        .map_err(Max31865Err::Interface)
}

pub(crate) fn write<S: SpiDevice<u8>>(
    spi: &mut S,
    address: u8,
    value: u8,
) -> Result<(), Max31865Err<S::Error>> {
    spi.write(&[address | 0x80, value])
        .map_err(Max31865Err::Interface)
}
