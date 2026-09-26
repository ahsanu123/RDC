use bilge::prelude::*;
use bitflags::bitflags;
use embedded_hal::{delay::DelayNs, spi::SpiDevice};

use crate::{communication::possible_address::PossibleAddress, device::DeviceTrait};

bitflags! {
    // address = 00H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    pub struct StatusRegisterBits: u16{
                                                          //FEDC_BA98_7654_3210
        const READ_STATUS_RU                            = 0b1000_0000_0000_0000;
        const SLAVE_NUMBER_W                            = 0b0110_0000_0000_0000;
        const NO_GMR_ANGLE_VAL_RU                       = 0b0001_0000_0000_0000;
        const NO_GMR_XY_VAL_RU                          = 0b0000_1000_0000_0000;
        const STATUS_ROM_R                              = 0b0000_0100_0000_0000;
        const STATUS_ADC_TEST_R                         = 0b0000_0010_0000_0000;

        const STATUS_MAGNITUDE_OUT_OF_LIMIT_RU          = 0b0000_0000_1000_0000;
        const STATUS_XY_OUT_OF_LIMIT_RU                 = 0b0000_0000_0100_0000;
        const STATUS_OVERFLOW_RU                        = 0b0000_0000_0010_0000;
        const STATUS_DIGITAL_SIGNAL_PROCESSING_UNIT_R   = 0b0000_0000_0001_0000;
        const STATUS_FUSE_CRC_R                         = 0b0000_0000_0000_1000;
        const STATUS_VOLTAGE_REGULATOR_R                = 0b0000_0000_0000_0100;
        const STATUS_WATCHDOG_R                         = 0b0000_0000_0000_0010;
        const STATUS_RESET_R                            = 0b0000_0000_0000_0001;
    }
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ReadStatus {
    NoChange = 0,
    Changed,
}

#[bitsize(2)]
#[derive(FromBits)]
pub enum SlaveNumber {
    Num1 = 0,
    Num2,
    Num3,
    Num4,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum GMRAngleVal {
    Valid = 0,
    Invalid,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum GMRXYVal {
    Valid = 0,
    Invalid,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum StatusROM {
    CRCOk = 0,
    CRCFailOrRunning,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum StatusADCTest {
    Ok = 0,
    OutOfLimit,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum StatusGMRMagnitude {
    Ok = 0,
    OutOfLimit,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum StatusXYDataOut {
    Ok = 0,
    //  X,Y data out of limit (>23230 digits, <-23230 digits)
    OutOfLimit,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum StackOverflow {
    NoDSPUOverflow = 0,
    DSPUOverflowOccured,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum StatusDSPUnit {
    Ok = 0,
    NotOkOrRunning,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum StatusFuseCRC {
    Ok = 0,
    Fail,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum StatusVoltageRegulator {
    Ok = 0,
    OverVoltage,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum StatusWatchdog {
    Normal = 0,
    CounterExpired,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum StatusReset {
    NoResetSinceLastReadOut = 0,
    ResetHappen,
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct StatusRegisterStructure {
    pub status_reset: StatusReset,
    pub status_watchdog: StatusWatchdog,
    pub status_voltage_regulator: StatusVoltageRegulator,
    pub status_fuse_crc: StatusFuseCRC,
    pub status_dsp_unit: StatusDSPUnit,
    pub status_stack_overflow: StackOverflow,
    pub status_xy_data_out: StatusXYDataOut,
    pub status_gmr_magnitude: StatusGMRMagnitude,

    //reserved
    reserved: u1,

    pub status_adc_test: StatusADCTest,
    pub status_rom: StatusROM,
    pub status_gmr_xy_val: GMRXYVal,
    pub status_gmr_angle_val: GMRAngleVal,
    pub slave_number: SlaveNumber,
    pub read_status: ReadStatus,
}

pub trait StatusRegisterHandler<SPI, DELAY, DEVICE>
where
    SPI: SpiDevice,
    DELAY: DelayNs,
    DEVICE: DeviceTrait<SPI, DELAY>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<StatusRegisterStructure, SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

pub struct StatusRegister {}

impl<SPI, DELAY, DEVICE> StatusRegisterHandler<SPI, DELAY, DEVICE> for StatusRegister
where
    SPI: SpiDevice,
    DELAY: DelayNs,
    DEVICE: DeviceTrait<SPI, DELAY>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<StatusRegisterStructure, <SPI>::Error> {
        let data = dev.read(PossibleAddress::StatusRegister)?;
        let safety_word = (data as u16);
        // TODO: remove expect
        let status = StatusRegisterStructure::from(u16::new((data >> 16) as u16));

        Ok(status)
    }

    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), <SPI>::Error> {
        todo!()
    }
}
