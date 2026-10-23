//! Wire framing: Rev A, pages 5-8 and 17.

use embedded_hal::{i2c::I2c, spi::SpiDevice};

use crate::register::{Address, I2C_ADDRESS};

mod sealed {
    pub trait Sealed {}
}

/// Register transport implemented by the supplied I2C and four-wire SPI adapters.
pub trait RegisterInterface: sealed::Sealed {
    type Error;
    #[doc(hidden)]
    fn read(&mut self, address: Address, bytes: &mut [u8]) -> Result<(), Self::Error>;
    #[doc(hidden)]
    fn write(&mut self, address: Address, value: u8) -> Result<(), Self::Error>;
}

/// I2C adapter at the fixed seven-bit address 0x30.
pub struct I2cInterface<I> {
    bus: I,
}

impl<I> I2cInterface<I> {
    pub fn new(bus: I) -> Self {
        Self { bus }
    }

    pub fn release(self) -> I {
        self.bus
    }
}

impl<I: I2c> sealed::Sealed for I2cInterface<I> {}

impl<I: I2c> RegisterInterface for I2cInterface<I> {
    type Error = I::Error;

    fn read(&mut self, address: Address, bytes: &mut [u8]) -> Result<(), Self::Error> {
        self.bus.write_read(I2C_ADDRESS, &[address as u8], bytes)
    }

    fn write(&mut self, address: Address, value: u8) -> Result<(), Self::Error> {
        self.bus.write(I2C_ADDRESS, &[address as u8, value])
    }
}

/// Four-wire SPI adapter; the supplied SpiDevice manages CS and shared-bus locking.
pub struct SpiInterface<S> {
    device: S,
}

impl<S> SpiInterface<S> {
    pub fn new(device: S) -> Self {
        Self { device }
    }

    pub fn release(self) -> S {
        self.device
    }
}

impl<S: SpiDevice<u8>> sealed::Sealed for SpiInterface<S> {}

impl<S: SpiDevice<u8>> RegisterInterface for SpiInterface<S> {
    type Error = S::Error;

    fn read(&mut self, address: Address, bytes: &mut [u8]) -> Result<(), Self::Error> {
        let command = [0x80 | address as u8];
        self.device.transaction(&mut [
            embedded_hal::spi::Operation::Write(&command),
            embedded_hal::spi::Operation::Read(bytes),
        ])
    }

    fn write(&mut self, address: Address, value: u8) -> Result<(), Self::Error> {
        self.device.write(&[address as u8, value])
    }
}
