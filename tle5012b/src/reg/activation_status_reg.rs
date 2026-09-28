use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//     Res 15:11 res Reserved, Reset: 00011B (during operation may change to 01011B)
//
//     AS_FRST 10 wu Activation of Firmware Reset
//         All configuration registers retain their contents.
//         0B default or after execution of firmware reset.
//         Firmware also sets S_RST at this point.
//         1B activation of firmware reset.
//         Reset: 0B
//
//     AS_ADCT 9 wu Enable ADC Test vector Check
//         Activation of this test is only allowed with deactivated
//         AUTOCAL. X, Y and Temp channel will be checked.
//         0B after execution.
//         1B activation of ADC Test vector Check.
//         Reset: 1B (for update buffer 0B if no update command
//         send before)
//     AS_VEC_MAG 7 wu Activation of Magnitude Check
//         0B monitoring of magnitude disabled1).
//         1B monitoring of magnitude enabled.
//         Reset: 1B (for update buffer 0B if no update command
//         send before)
//     AS_VEC_XY 6 wu Activation of X,Y Out of Limit-Check
//         0B monitoring of X,Y Out of Limit disabled1).
//         1B monitoring of X,Y Out of Limit enabled.
//         Reset: 1B (for update buffer 0B if no update command
//         send before)
//     AS_OV 5 wu Enable of DSPU Overflow Check
//         0B monitoring of DSPU Overflow disabled1).
//         1B monitoring of DSPU Overflow enabled.
//         Reset: 1B (for update buffer 0B if no update command
//         send before)
//     AS_DSPU 4 wu Activation DSPU BIST
//         0B after execution
//         1B activation of DSPU BIST or BIST running
//         Reset: 1B (for update buffer 0B if no update command
//         send before)
//     AS_FUSE 3 wu Activation Fuse CRC
//         A write in any of the fuse registers will set this bit
//         automatically (automatically enabled by deactivation of
//         AUTOCAL). AUTOCAL disables register CRC check
//         regardless of the AS_FUSE setting.
//         0B monitoring of CRC disabled. Clearing this
//         activation bit will also disable reporting of S_FUSE
//         errors after a remaining error has been read-out.
//         1B monitoring of CRC enabled
//         Reset: 1B (for update buffer 0B if no update command
//         send before)
//     AS_VR 2 wu Enable Voltage Regulator Check
//         0B check of regulator voltages disabled. Clearing this
//         activation bit will also disable reporting of S_VR
//         error after a remaining error has been read-out.
//         1B check of regulator voltages enabled
//         Reset: 1B (for update buffer 0B if no update command
//         send before)
//     AS_WD 1 wu Enable DSPU Watchdog
//         0B DSPU watchdog monitoring disabled. The S_WD
//         status will be immediately cleared, when this bit is
//         cleared.
//         1B DSPU Watchdog monitoring enabled.
//         Reset: 1B (for update buffer 0B if no update command
//         send before)
//     AS_RST 0 w Activation of Hardware Reset
//         Activation occurs after CSQ switches from ’0’ to ’1’ after
//         SSC transfer.
//         0B after execution (write only, thus always returns “0”).
//         1B activation of HW Reset (S_RST is set).
//         Reset: 0B

bitflags! {
    // address = 01H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct ActivationStatusRegister : u16{
                                                                  //FEDC_BA98_7654_3210
        const ACTIVATION_OF_FIRMWARE_RESET_WU                   = 0b0000_0100_0000_0000;
        const EN_ADC_TEST_VECTOR_CHECK_WU                       = 0b0000_0010_0000_0000;
        const ACTIVATION_MAGNITUDE_CHECK_WU                     = 0b0000_0000_1000_0000;
        const ACTIVATION_XY_OUT_LIMIT_CHECK_WU                  = 0b0000_0000_0100_0000;
        const ENABLE_OF_DSPU_OVERFLOW_CHECK_WU                  = 0b0000_0000_0010_0000;

        const ACTIVATION_DSPU_BIST                              = 0b0000_0000_0001_0000;
        const ACTIVATION_FUSE_CRC                               = 0b0000_0000_0000_1000;
        const ENABLE_VOLTAGE_REGULATOR_CHECK                    = 0b0000_0000_0000_0100;
        const ENABLE_DSPU_WATCHDOG                              = 0b0000_0000_0000_0010;
        const ACTIVATION_OF_HARDWARE_RESET                      = 0b0000_0000_0000_0001;
    }
}

pub trait ActivationStatusRegisterHandler<SPI, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DEVICE> ActivationStatusRegisterHandler<SPI, DEVICE> for ActivationStatusRegister
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

impl RegisterFromRaw for ActivationStatusRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::ActivationStatusRegister;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct ActivationStatusRegisterStructure {
    activation_of_hw_rst: ActivationOfHardwareReset,
    enable_dspu_watchdog: EnableDSPUWatchdog,
    enable_voltage_regulator_check: EnableVoltageRegulatorCheck,
    activation_fuse_crc: ActivationFuseCRC,
    activation_dspu_bist: ActivationDSPUBIST,
    enable_dspu_overflow_check: EnableDSPUOverflowCheck,
    activation_xy_out_limit_check: ActivationXYOutLimitCheck,
    activation_magnitude_check: ActivationMagnitudeCheck,
    reserved_8: u1,
    enable_adc_test_vector_check: EnableADCTestVectorCheck,
    activation_of_firmware_reset: ActivationOfFirmwareReset,
    reserved_15_11: u5,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ActivationOfHardwareReset {
    AfterExecution = 0,
    ActivationOfHwReset,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum EnableDSPUWatchdog {
    Disable = 0,
    Enable,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum EnableVoltageRegulatorCheck {
    Disable = 0,
    Enable,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ActivationFuseCRC {
    MonitoringDisable = 0,
    MonitoringEnable,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ActivationDSPUBIST {
    AfterExecution = 0,
    RunningReset,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum EnableDSPUOverflowCheck {
    Disable = 0,
    Enable,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ActivationXYOutLimitCheck {
    Disable = 0,
    Enable,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ActivationMagnitudeCheck {
    Disable = 0,
    Enable,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum EnableADCTestVectorCheck {
    AfterExecution = 0,
    Activation,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ActivationOfFirmwareReset {
    DefaultOrAfterExecution = 0,
    Activation,
}
