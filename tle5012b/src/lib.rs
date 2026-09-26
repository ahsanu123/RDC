#![no_std]

use crate::reg::status_reg::StatusRegister;
pub mod communication;
pub(crate) mod crc_table;
pub mod device;
pub(crate) mod reg;

pub struct TLE5012B<SPI, DELAY>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
{
    spi: SPI,
    delay: DELAY,
    status_reg: StatusRegister,
}

impl<SPI, DELAY> TLE5012B<SPI, DELAY>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
{
    pub fn new(spi: SPI, delay: DELAY) -> Self {
        Self {
            spi,
            delay,
            status_reg: StatusRegister {},
        }
    }
}
