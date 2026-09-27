use crate::device::DeviceTrait;
use bilge::prelude::*;
use bitflags::bitflags;

bitflags! {
    // address = 11H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct ADCYRawValueRegister: u16{
                               //FEDC_BA98_7654_3210
        const Y_RAW_VALUE    = 0b1111_1111_1111_1111;

    }
}

pub trait ADCYRawValueRegisterHandler<SPI, DELAY, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
    DEVICE: DeviceTrait<SPI, DELAY>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DELAY, DEVICE> ADCYRawValueRegisterHandler<SPI, DELAY, DEVICE> for ADCYRawValueRegister
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
    DEVICE: DeviceTrait<SPI, DELAY>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error> {
        todo!()
    }

    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error> {
        todo!()
    }
}

#[bitsize(16)]
#[derive(FromBits)]
pub struct ADCYRawValueRegisterStructure {
    raw_value: u16,
}
