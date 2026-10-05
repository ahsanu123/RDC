use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// T_TGL 15 ru Temperature Sensor Raw-Value Toggle
//     Toggles after every new temperature value (T_RAW).
//     Reset: 0B
// T_RAW 9:0 ru Temperature Sensor Raw-Value
//     Temperature at ADC. This value is not compensated with
//     the offset temperature. T_RAW range is not limited as
//     TEMPER. T_RAW is an unsigned value.
//     T[°C]=(T_RAW[dig]-369[dig]-T25O[dig]) / 2.776[dig/°C]
//     Reset: 0H

bitflags! {
    // address = 15H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct TemperatureSensorRawValueRegister: u16{
                                                          //FEDC_BA98_7654_3210
        const TEMPERATURE_SENSOR_RAW_VALUE_TOGGLE_RU    = 0b1000_0000_0000_0000;
        const TEMPERATURE_SENSOR_RAW_VALUE_RU           = 0b0000_0011_1111_1111;

    }
}

#[allow(unused)]
pub trait TemperatureSensorRawValueRegisterHandler<SPI, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DEVICE> TemperatureSensorRawValueRegisterHandler<SPI, DEVICE>
    for TemperatureSensorRawValueRegister
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

impl RegisterFromRaw for TemperatureSensorRawValueRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::TemperatureSensorRawValue;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct TemperatureSensorRawValueRegisterStructure {
    temperature_sensor_raw_value: u10,
    reserved_14_10: u5,
    temperature_sensor_raw_value_toggle: TemperatureSensorRawValueToggle,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum TemperatureSensorRawValueToggle {
    ToggleState0 = 0,
    ToggleState1,
}
