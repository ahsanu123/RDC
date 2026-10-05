use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// MAG 9:0 ru Angle Vector Magnitude
//     Unsigned Angle Vector Magnitude after X, Y error
//     compensation (due to temperature).
//     This field allows additional safety checks.
//     Formula:
//     MAG = (SQRT(X*X+Y*Y))/64
//     Reset: 0H

bitflags! {
    // address = 11H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct AngleVectorMagnitudeRegister: u16{
                                           //FEDC_BA98_7654_3210
        const ANGLE_VECTOR_MAGNITUDE_RU  = 0b0000_0011_1111_1111;

    }
}

#[allow(unused)]
pub trait AngleVectorMagnitudeRegisterHandler<SPI, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DEVICE> AngleVectorMagnitudeRegisterHandler<SPI, DEVICE> for AngleVectorMagnitudeRegister
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

impl RegisterFromRaw for AngleVectorMagnitudeRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::AngleVectorMagnitude;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct AngleVectorMagnitudeRegisterStructure {
    angle_vector_magnitude: u10,
    reserved_15_10: u6,
}
