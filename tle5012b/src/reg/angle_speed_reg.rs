use crate::{
    communication::possible_address::PossibleAddress,
    device::{DeviceError, DeviceTrait},
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// RD_AS 15 r Read Status, Angle Speed
//     0B no new angle speed value since last readout
//     1B new angle speed value (ANG_SPD) present. The
//     bit is cleared on a read-out (valid for both: normal
//     operation and update buffer). Note: If an update
//     event (register snapshot) is done after a normal
//     read, RD_AS will not be set to 1B in the following
//     read (either update read or normal read) unless a
//     new value is available.
//     Reset: 1B
// ANG_SPD 14:0 ru Calculated Angle Speed
//     Signed value, where the sign bit [14] indicates the
//     direction of the rotation.
//     Without prediction difference between the current
//     unpredicted angle value and second-to-last unpredicted
//     angle values.

bitflags! {
    // address = 03H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct AngleSpeedRegister: u16{
                                                //FEDC_BA98_7654_3210
        const READ_STATUS_ANGLE_SPEED           = 0b1000_0000_0000_0000;
        const CALCULATED_ANGLE_SPEED_15BIT_RU   = 0b0111_1111_1111_1111;
    }
}

#[allow(unused)]
pub trait AngleSpeedRegisterHandler<SPI, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_angular_speed(&mut self, dev: &mut DEVICE) -> Result<u15, DeviceError<SPI::Error>>;
    fn get_angular_speed_reg(
        &mut self,
        dev: &mut DEVICE,
    ) -> Result<AngleSpeedRegisterStructure, DeviceError<SPI::Error>>;
}

impl<SPI, DEVICE> AngleSpeedRegisterHandler<SPI, DEVICE> for AngleSpeedRegister
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_angular_speed(&mut self, dev: &mut DEVICE) -> Result<u15, DeviceError<<SPI>::Error>> {
        let raw_val = dev.read(PossibleAddress::AngleSpeedRegister)?;
        let raw_val = (raw_val >> 16) as u16;

        let parsed_val = AngleSpeedRegisterStructure::from(raw_val);

        Ok(parsed_val.calculated_angle_speed())
    }

    fn get_angular_speed_reg(
        &mut self,
        dev: &mut DEVICE,
    ) -> Result<AngleSpeedRegisterStructure, DeviceError<<SPI>::Error>> {
        let raw_val = dev.read(PossibleAddress::AngleSpeedRegister)?;
        let raw_val = (raw_val >> 16) as u16;

        let parsed_val = AngleSpeedRegisterStructure::from(raw_val);
        Ok(parsed_val)
    }
}

impl RegisterFromRaw for AngleSpeedRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::AngleSpeedRegister;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct AngleSpeedRegisterStructure {
    calculated_angle_speed: u15,
    read_status_angle_speed: ReadStatusAngleSpeed,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ReadStatusAngleSpeed {
    NoNewAngleSpeedValue = 0,
    NewAngleSpeedValuePresent,
}
