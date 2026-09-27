use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// SYNCH 15:4 w Amplitude Synchronicity
//     12-bit signed integer value of amplitude synchronicity
//     correction (raw X amplitude divided by raw Y amplitude).
//     For synchronicity correction, the offset compensated Y
//     value is multiplied by SYNCH.
//     +2047D 112.494%
//     0D 100%
//     -2048D 87.500%
//     Reset: device-specific

bitflags! {
    // address = 0CH
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct SynchronicityRegister: u16{
                                          //FEDC_BA98_7654_3210
        const AMPLITUDE_SYNCHRONICITY_W = 0b1111_1111_1111_0000;
    }
}

pub trait SynchronicityRegisterHandler<SPI, DELAY, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
    DEVICE: DeviceTrait<SPI, DELAY>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DELAY, DEVICE> SynchronicityRegisterHandler<SPI, DELAY, DEVICE> for SynchronicityRegister
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

impl RegisterFromRaw for SynchronicityRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::Synchronicity;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct SynchronicityRegisterStructure {
    reserved_3_0: u4,
    amplitude_synchronicity_correction: u12,
}
