use crate::{
    communication::possible_address::PossibleAddress,
    device::{DeviceError, DeviceTrait},
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
    DEVICE: DeviceTrait<SPI>,
    SPI: embedded_hal::spi::SpiDevice,
{
    fn read_raw_angval(&mut self, dev: &mut DEVICE) -> Result<u15, DeviceError<SPI::Error>>;
    fn new_value_exist(&mut self, dev: &mut DEVICE) -> Result<bool, DeviceError<SPI::Error>>;
    fn get_reg_value(
        &mut self,
        dev: &mut DEVICE,
    ) -> Result<AngleValueRegisterStructure, DeviceError<SPI::Error>>;
}

impl<SPI, DEVICE> AngleValueRegisterHandler<SPI, DEVICE> for AngleValueRegister
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_raw_angval(&mut self, dev: &mut DEVICE) -> Result<u15, DeviceError<SPI::Error>> {
        let raw_val = dev.read(PossibleAddress::AngleValueRegister)?;
        let raw_val = (raw_val >> 16) as u16;

        let parsed_val = AngleValueRegisterStructure::from(raw_val);

        Ok(parsed_val.calculated_angle_value())
    }

    fn new_value_exist(&mut self, dev: &mut DEVICE) -> Result<bool, DeviceError<<SPI>::Error>> {
        let raw_val = dev.read(PossibleAddress::AngleValueRegister)?;
        let raw_val = (raw_val >> 16) as u16;
        let val = AngleValueRegisterStructure::from(raw_val);

        match val.read_status_angle_value() {
            ReadStatusAngleValue::NoNewAngleValue => Ok(false),
            ReadStatusAngleValue::NewAngleValuePresent => Ok(true),
        }
    }

    fn get_reg_value(
        &mut self,
        dev: &mut DEVICE,
    ) -> Result<AngleValueRegisterStructure, DeviceError<<SPI>::Error>> {
        let raw_val = dev.read(PossibleAddress::AngleValueRegister)?;

        Ok(AngleValueRegisterStructure::from((raw_val >> 16) as u16))
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
    pub calculated_angle_value: u15, // require calculation
    pub read_status_angle_value: ReadStatusAngleValue,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ReadStatusAngleValue {
    NoNewAngleValue = 0,
    NewAngleValuePresent,
}
