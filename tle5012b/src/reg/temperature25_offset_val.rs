use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Descri
//
// T25O 15:9 r Temperature 25°C Offset value
//     Signed offset value at 25°C temperature; 1dig=0.36°C.
//     T25O = T_RAW(@25°C)[dig]-439[dig].
//     Reset: device-specific

bitflags! {
    // address = 30H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct Temperature25OffsetValueRegister: u16{
                                            //FEDC_BA98_7654_3210
        const TEMPERATURE_25_OFFSET_VALUE = 0b1111_1110_0000_0000;

    }
}

pub trait Temperature25OffsetValueRegisterHandler<SPI, DELAY, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
    DEVICE: DeviceTrait<SPI, DELAY>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DELAY, DEVICE> Temperature25OffsetValueRegisterHandler<SPI, DELAY, DEVICE>
    for Temperature25OffsetValueRegister
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

impl RegisterFromRaw for Temperature25OffsetValueRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::Temperature25cOffsetValue;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct Temperature25OffsetValueRegisterStructure {
    reserved_8_0: u9,
    offset_value: u7,
}
