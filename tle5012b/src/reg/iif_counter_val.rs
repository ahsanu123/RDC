use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// IIF_CNT 13:0 ru Counter value of increments
//     Internal 14-bit counter for the incremental interface,
//     which counts from 0 to 16383 during one full turn.
//     It can be used for synchronization purposes between
//     sensor and counter value on microcontroller side.
//     Therefore, depending on the setting of the IFAB_RES
//     register (9bit to 12bit resolution of incremental interface),
//     2 to 5 LSBs have to be removed from IIF_CNT for the
//     synchronization.
//     Reset: 0H

bitflags! {
    // address = 20H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct IIFCounterValueRegister: u16{
                                           //FEDC_BA98_7654_3210
        const COUNTER_VALUE_OF_INCREMENT = 0b0011_1111_1111_1111;

    }
}

pub trait IIFCounterValueRegisterHandler<SPI, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DEVICE> IIFCounterValueRegisterHandler<SPI, DEVICE> for IIFCounterValueRegister
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

impl RegisterFromRaw for IIFCounterValueRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::IifCounterValue;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct IIFCounterValueRegisterStructure {
    increments_value: u14,
    reserved_15_14: u2,
}
