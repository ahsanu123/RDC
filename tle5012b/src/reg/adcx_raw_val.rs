use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// ADC_X 15:0 r ADC value of X-GMR
//     16-bit signed integer raw X value. Read-out of this
//     register will update ADC_Y
//     Reset: 0H

bitflags! {
    // address = 10H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct ADCXRawValueRegister: u16{
                               //FEDC_BA98_7654_3210
        const X_RAW_VALUE    = 0b1111_1111_1111_1111;

    }
}

#[allow(unused)]
pub trait ADCXRawValueRegisterHandler<SPI, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DEVICE> ADCXRawValueRegisterHandler<SPI, DEVICE> for ADCXRawValueRegister
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error> {
        todo!()
    }

    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error> {
        todo!()
    }
}

impl RegisterFromRaw for ADCXRawValueRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::AdcXRawValue;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
pub struct ADCXRawValueRegisterStructure {
    raw_value: u16,
}
