use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// FSYNC 15:9 wu Frame Synchronization Counter Value
//     Subcounter within one frame. Increments every internal
//     clock cycle (synchronously at a 750kHz rate). Maximum
//     counter value depends on FIR_MD setting: 16 @
//     FIR_MD=00; 32 @ FIR_MD=01; 64 @ FIR_MD=10; 128
//     @ FIR_MD=11.
//     Reset: 0H
// TEMPER 8:0 ru Temperature Value
//     Signed offset compensated temperature value. Saturated
//     below approx. -30°C and above approx. +140°C.
//     Compensation done by DSPU from T_RAW and the
//     offset temperature T25O.
//     T[°C] = (TEMPER[dig]+161[dig]) / 2.776[dig/°C]
//     For reference point on the real temperature the voltage
//     via the ESD diode at VDD pin is used. This introduces
//     some variation from device to device. After
//     characterization, a 9-bit correction is considered more
//     accurate to extract the temperature:
//     T[°C] = (TEMPER[dig]+152[dig]) / 2.776[dig/°C]
//     Reset: 0H

bitflags! {
    // address = 05H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct FrameSyncRegister: u16{
                                                          //FEDC_BA98_7654_3210
        const FRAME_SYNCHRONIZATION_COUNTER_VALUE_WU    = 0b1111_1110_0000_0000;
        const TEMPERATURE_VALUE_RU                      = 0b0000_0001_1111_1111;
    }
}

pub trait FrameSyncRegisterHandler<SPI, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DEVICE> FrameSyncRegisterHandler<SPI, DEVICE> for FrameSyncRegister
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

impl RegisterFromRaw for FrameSyncRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::FrameSynchronizationRegister;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct FrameSyncRegisterStructure {
    temperature_value: u9,
    frame_synchronization_counter_value: u7,
}
