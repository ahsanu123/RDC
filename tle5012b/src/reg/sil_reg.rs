use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// FILT_PAR 15 w Filter Parallel
//     Diagnostic function to test ADCs’ filter. If enabled, the raw
//     X-signal is routed also to the raw Y-signal input of the
//     filter so SIN and COS signal should be identical.
//     0B filter parallel disabled
//     1B filter parallel enabled (source: X-value)
//     Reset: 0B
// FILT_INV 14 w Filter Inverted
//     Diagnostic function to test ADCs’ filter. If enabled, the Xand Y-signals are inverted. The angle output is then
//     shifted by 180°.
//     0B filter inverted disabled
//     1B filter inverted enabled
//     Reset: 0B
// FUSE_REL 10 w Fuse Reload
//     Triggers reload of default values from laser fuses into
//     configuration registers.
//     0B normal operation
//     1B reload of registers with fuse values immediately.
//     Reloaded fuse values are used with the start of the
//     next filter cycle.
//     Reset: 0B
// ADCTV_EN 6 w ADC-Test Vectors
//     Diagnostic function to test ADCs. If enabled, sensor
//     elements are internally disconnected and test voltages
//     are connected to ADCs. NO_GMR_A and NO_GMR_XY
//     status flags will be set to “1”, if this bit is set during
//     operation. Test vectors can be selected via the register
//     ADCTV_Y and ADCTV_X.
//     0B ADC-Test Vectors disabled
//     1B ADC-Test Vectors enabled
//     Reset: 0B
// ADCTV_Y 5:3 w Test vector Y
//     000B 0V
//     001B +70%
//     010B +100%
//     011B +Overflow
//     101B -70%
//     110B -100%
//     111B -Overflow
//     Reset: 0H
// ADCTV_X 2:0 w Test vector X
//     000B 0V
//     001B +70%
//     010B +100%
//     011B +Overflow
//     101B -70%
//     110B -100%
//     111B -Overflow
//     Reset: 0H

bitflags! {
    // address = 07H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct SILRegister: u16{
                                   //FEDC_BA98_7654_3210
        const FILTER_PARALLEL_W  = 0b1000_0000_0000_0000;
        const FILTER_INVERTED_W  = 0b0100_0000_0000_0000;
        const FUSE_RELOAD_W      = 0b0000_0100_0000_0000;
        const ADC_TEST_VECTORS_W = 0b0000_0000_0100_0000;
        const TEST_VECTOR_Y_W    = 0b0000_0000_0011_1000;
        const TEST_VECTOR_X_W    = 0b0000_0000_0000_0111;
    }
}

pub trait SILRegisterHandler<SPI, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DEVICE> SILRegisterHandler<SPI, DEVICE> for SILRegister
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

impl RegisterFromRaw for SILRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::SilRegister;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct SILRegisterStructure {
    test_vector_x: ADCTestVector,
    test_vector_y: ADCTestVector,
    adc_test_vectors: ADCTestVectors,
    reserved_9_7: u3,
    fuse_reload: FuseReload,
    reserved_13_11: u3,
    filter_inverted: FilterInverted,
    filter_parallel: FilterParallel,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum FilterParallel {
    Disabled = 0,
    EnabledSourceXValue,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum FilterInverted {
    Disabled = 0,
    Enabled,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum FuseReload {
    NormalOperation = 0,
    ReloadRegistersWithFuseValues,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ADCTestVectors {
    Disabled = 0,
    Enabled,
}

#[bitsize(3)]
#[derive(FromBits)]
pub enum ADCTestVector {
    ZeroVolt = 0,
    Positive70Percent,
    Positive100Percent,
    PositiveOverflow,
    Reserved,
    Negative70Percent,
    Negative100Percent,
    NegativeOverflow,
}
