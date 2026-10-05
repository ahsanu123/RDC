#![no_std]

pub mod communication;
pub(crate) mod crc_table;
pub mod device;
pub mod models;
pub(crate) mod reg;
pub(crate) mod signed_conversion;

use crate::{device::DeviceInner, reg::prelude::*};

pub struct TLE5012B<SPI>
where
    SPI: embedded_hal::spi::SpiDevice,
{
    inner: DeviceInner<SPI>,

    pub status_reg: StatusRegister,
    pub activation_status_reg: ActivationStatusRegister,
    pub angle_value_reg: AngleValueRegister,
    pub angle_speed_reg: AngleSpeedRegister,
    pub angle_revolution_reg: AngleRevolutionRegister,
    pub frame_sync_reg: FrameSyncRegister,
    pub mode1_reg: Mode1Register,
    pub sil_reg: SILRegister,
    pub mode2_reg: Mode2Register,
    pub mode3_reg: Mode3Register,
    pub offset_x_reg: OffsetXRegister,
    pub offset_y_reg: OffsetYRegister,
    pub synchronicity_reg: SynchronicityRegister,
    pub ifab_reg: IFABRegister,
    pub mode4_reg: Mode4Register,
    pub temperature_coefficient_reg: TemperatureCoefficientRegister,
    pub angle_vector_magnitude_reg: AngleVectorMagnitudeRegister,
    pub temperature_sensor_raw_value_reg: TemperatureSensorRawValueRegister,
    pub iif_counter_value_reg: IIFCounterValueRegister,
    pub temperature_25_offset_value_reg: Temperature25OffsetValueRegister,
}

impl<SPI> TLE5012B<SPI>
where
    SPI: embedded_hal::spi::SpiDevice,
{
    pub fn new(spi: SPI) -> Self {
        let inner = DeviceInner::new(spi);

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
}
