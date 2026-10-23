//! Blocking I2C and SPI register transactions.

use embedded_hal::{
    i2c::I2c,
    spi::{Operation, SpiDevice},
};

use crate::{
    register::{Configuration, Register},
    u3,
};

const I2C_BASE_ADDRESS: u8 = 0x20;
const SPI_BASE_OPCODE: u8 = 0x40;
const SPI_READ: u8 = 0x01;
const SPI_A2: u8 = 0x08;
const IOCON_HAEN: u8 = 0x08;

mod sealed {
    pub trait Sealed {}
}

/// Register transport, sealed to preserve device addressing invariants.
pub trait Interface: sealed::Sealed {
    type Error;
    const REQUIRES_HAEN: bool;

    /// Prepare a device in its power-on register map.
    fn initialize(&mut self) -> Result<(), Self::Error>;
    fn read(&mut self, address: u8) -> Result<u8, Self::Error>;
    fn write(&mut self, address: u8, value: u8) -> Result<(), Self::Error>;
}

/// MCP23017 I2C transport; address is 0x20 plus the three address-pin levels.
/// Datasheet: DS20001952C, sections 3.2.2 and 3.3.1, I2C interface and addressing.
pub struct I2cInterface<I2C> {
    bus: I2C,
    address: u8,
}

impl<I2C> I2cInterface<I2C> {
    pub fn new(bus: I2C, address_pins: u3) -> Self {
        Self {
            bus,
            address: I2C_BASE_ADDRESS | address_pins.value(),
        }
    }

    pub fn release(self) -> I2C {
        self.bus
    }
}

impl<I2C> sealed::Sealed for I2cInterface<I2C> {}

impl<I2C: I2c> Interface for I2cInterface<I2C> {
    type Error = I2C::Error;
    const REQUIRES_HAEN: bool = false;

    fn initialize(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn read(&mut self, address: u8) -> Result<u8, Self::Error> {
        let mut data = [0];
        self.bus.write_read(self.address, &[address], &mut data)?;
        Ok(data[0])
    }

    fn write(&mut self, address: u8, value: u8) -> Result<(), Self::Error> {
        self.bus.write(self.address, &[address, value])
    }
}

/// MCP23S17 SPI transport. `SpiDevice` keeps CS asserted for the entire read.
/// Datasheet: DS20001952C, sections 3.2.3 and 3.3.2; erratum DS80311A, issue 1.
pub struct SpiInterface<SPI> {
    bus: SPI,
    opcode: u8,
}

impl<SPI> SpiInterface<SPI> {
    pub fn new(bus: SPI, address_pins: u3) -> Self {
        Self {
            bus,
            opcode: SPI_BASE_OPCODE | (address_pins.value() << 1),
        }
    }

    pub fn release(self) -> SPI {
        self.bus
    }
}

impl<SPI> sealed::Sealed for SpiInterface<SPI> {}

impl<SPI: SpiDevice<u8>> Interface for SpiInterface<SPI> {
    type Error = SPI::Error;
    const REQUIRES_HAEN: bool = true;

    fn initialize(&mut self) -> Result<(), Self::Error> {
        // DS80311A: reset silicon with A2 high requires an opcode with A2 set.
        // Broadcast to both groups before selecting the hardware address.
        self.bus
            .write(&[SPI_BASE_OPCODE, Configuration::ADDRESS_A, IOCON_HAEN])?;
        self.bus.write(&[
            SPI_BASE_OPCODE | SPI_A2,
            Configuration::ADDRESS_A,
            IOCON_HAEN,
        ])
    }

    fn read(&mut self, address: u8) -> Result<u8, Self::Error> {
        let header = [self.opcode | SPI_READ, address];
        let mut data = [0];
        self.bus
            .transaction(&mut [Operation::Write(&header), Operation::Read(&mut data)])?;
        Ok(data[0])
    }

    fn write(&mut self, address: u8, value: u8) -> Result<(), Self::Error> {
        self.bus.write(&[self.opcode, address, value])
    }
}
