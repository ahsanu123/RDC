use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// RD_AV 15 r Read Status, Angle Value
//     0B no new angle value since last readout
//     1B new angle value (ANG_VAL) present. The bit is
//     cleared on a read-out (valid for both: normal
//     operation and update buffer). Note: If an update
//     event (register snapshot) is done after a normal
//     read, RD_AV will not be set to 1B in the following
//     read (either update read or normal read) unless a
//     new value is available.
//     Reset: 1B
// ANG_VAL 14:0 ru Calculated Angle Value (signed 15-bit)
//     (6.4)
//     4000H -180° (valid for ANG_RANGE = 0x080)
//     0000H 0°
//     3FFFH +179.99° (valid for ANG_RANGE = 0x080)
//     Reset: 0H

bitflags! {
    // address = 03H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct AngleValueRegister: u16{
                                                //FEDC_BA98_7654_3210
        const READ_STATUS_ANGLE_VALUE           = 0b1000_0000_0000_0000;
        const CALCULATED_ANGLE_VALUE_15BIT_RU   = 0b0111_1111_1111_1111;
    }
}

pub trait AngleValueRegisterHandler<SPI, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DEVICE> AngleValueRegisterHandler<SPI, DEVICE> for AngleValueRegister
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

impl RegisterFromRaw for AngleValueRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::AngleValueRegister;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct AngleValueRegisterStructure {
    calculated_angle_value: u15, // require calculation
    read_status_angle_value: ReadStatusAngleValue,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ReadStatusAngleValue {
    NoNewAngleValue = 0,
    NewAngleValuePresent,
}
