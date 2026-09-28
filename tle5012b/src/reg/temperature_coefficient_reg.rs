use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// TCO_Y_T 15:9 w Offset Temperature Coefficient for Y-Component
//     7-bit signed integer value of Y-offset temperature
//     coefficient. This register is used both with autocalibration
//     and without autocalibration. If autocalibration is
//     deactivated, overwrite only with the default value. If
//     autocalibration is activated, do not write this bitfield. See
//     “Offset temperature compensation” on Page 97.
//     Reset: device-specific
// SBIST 8 w Startup-BIST
//     0B Startup-BIST disabled
//     1B Startup-BIST enabled
//     Reset: 1B
// CRC_PAR 7:0 w CRC of Parameters
//     CRC of parameters from address 08H to 0FH. If any
//     settings within these registers are changed, this CRC has
//     to be changed accordingly.
//     Reset: device-specific

bitflags! {
    // address = 0FH
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct TemperatureCoefficientRegister: u16{
                                                                  //FEDC_BA98_7654_3210
        const OFFSET_TEMPERATURE_COEFFICIENT_FOR_Y_COMPONENT    = 0b1111_1100_0000_0000;
        const STARTUP_BIST                                      = 0b0000_0001_0000_0000;
        const CRC_OF_PARAMETERS                                 = 0b0000_0000_1111_1111;

    }
}

pub trait TemperatureCoefficientRegisterHandler<SPI, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DEVICE> TemperatureCoefficientRegisterHandler<SPI, DEVICE>
    for TemperatureCoefficientRegister
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

impl RegisterFromRaw for TemperatureCoefficientRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::TemperatureCoefficientReg;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct TemperatureCoefficientRegisterStructure {
    crc_of_parameters: u8,
    startup_bist: StartupBIST,
    y_temp_offset_coeff: u7,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum StartupBIST {
    Disabled = 0,
    Enabled,
}
