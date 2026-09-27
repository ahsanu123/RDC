#![no_std]

pub mod communication;
pub(crate) mod crc_table;
pub mod device;
pub(crate) mod reg;

use crate::{
    device::{DeviceInner, DeviceTrait},
    reg::prelude::*,
};

pub struct TLE5012B<SPI, DELAY>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
{
    inner: DeviceInner<SPI, DELAY>,

    status_reg: StatusRegister,
    activation_status_reg: ActivationStatusRegister,
    angle_value_reg: AngleValueRegister,
    angle_speed_reg: AngleSpeedRegister,
    angle_revolution_reg: AngleRevolutionRegister,
    frame_sync_reg: FrameSyncRegister,
    mode1_reg: Mode1Register,
    sil_reg: SILRegister,
    mode2_reg: Mode2Register,
    mode3_reg: Mode3Register,
    offset_x_reg: OffsetXRegister,
    offset_y_reg: OffsetYRegister,
    synchronicity_reg: SynchronicityRegister,
    ifab_reg: IFABRegister,
    mode4_reg: Mode4Register,
    temperature_coefficient_reg: TemperatureCoefficientRegister,
    angle_vector_magnitude_reg: AngleVectorMagnitudeRegister,
    temperature_sensor_raw_value_reg: TemperatureSensorRawValueRegister,
    iif_counter_value_reg: IIFCounterValueRegister,
    temperature_25_offset_value_reg: Temperature25OffsetValueRegister,
}

impl<SPI, DELAY> TLE5012B<SPI, DELAY>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
{
    pub fn new(spi: SPI, delay: DELAY) -> Self {
        let inner = DeviceInner::new(spi, delay);

        Self {
            inner,

            status_reg: StatusRegister::default(),
            activation_status_reg: ActivationStatusRegister::default(),
            angle_value_reg: AngleValueRegister::default(),
            angle_speed_reg: AngleSpeedRegister::default(),
            angle_revolution_reg: AngleRevolutionRegister::default(),
            frame_sync_reg: FrameSyncRegister::default(),
            mode1_reg: Mode1Register::default(),
            sil_reg: SILRegister::default(),
            mode2_reg: Mode2Register::default(),
            mode3_reg: Mode3Register::default(),
            offset_x_reg: OffsetXRegister::default(),
            offset_y_reg: OffsetYRegister::default(),
            synchronicity_reg: SynchronicityRegister::default(),
            ifab_reg: IFABRegister::default(),
            mode4_reg: Mode4Register::default(),
            temperature_coefficient_reg: TemperatureCoefficientRegister::default(),
            angle_vector_magnitude_reg: AngleVectorMagnitudeRegister::default(),
            temperature_sensor_raw_value_reg: TemperatureSensorRawValueRegister::default(),
            iif_counter_value_reg: IIFCounterValueRegister::default(),
            temperature_25_offset_value_reg: Temperature25OffsetValueRegister::default(),
        }
    }

    pub fn change_config(&mut self) {
        self.read_then_mutate::<StatusRegisterStructure>(
            communication::possible_address::PossibleAddress::StatusRegister,
            |status_reg| {
                status_reg.set_status_reset(StatusReset::ResetHappen);
            },
        );
    }
}
