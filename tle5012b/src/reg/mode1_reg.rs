use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// FIR_MD 15:14 w Update Rate Setting (Filter Decimation)
//     01B 42.7 µs
//     10B 85.3 µs
//     11B 170.6 µs
//     Reset: derivate-specific
// CLK_SEL 4 w Clock Source Select
//     Switch to external clock at start-up only. If there is no
//     clock signal on the IFC pin when the chip is switched to
//     the external clock source, the chip does not allow the
//     switch (CLK_SEL remains zero, operation continued). If
//     the external clock disappears with CLK_SEL already set,
//     the chip will reset (PLL out of lock) and run on with the
//     internal clock.
//     0B internal oscillator
//     1B external 4-MHz clock (IFC pin switched to input)
//     Reset: 0B
// DSPU_HOLD 2 w Hold DSPU Operation1)
//     If DSPU is on hold, no watchdog reset is performed by
//     DSPU. Deactivate watchdog with AS_WD before setting
//     DSPU on hold.
//     0B DSPU in normal schedule operation
//     1B DSPU is on hold
//     Reset: 0B
//     1) DSPU_HOLD is ignored in PWM or SPC mode.
// IIF_MOD 1:0 w Incremental Interface Mode
//     00B IIF disabled
//     01B A/B operation with Index on IFC pin
//     10B Step/Direction operation with Index on IFC pin
//     11B not allowed
//     Reset: derivate-specific

bitflags! {
    // address = 06H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct Mode1Register: u16{
                                              //FEDC_BA98_7654_3210
        const UPDATE_RATE_SETTING_W         = 0b1100_0000_0000_0000;
        const CLOCK_SOURCE_SELECT_W         = 0b0000_0000_0001_0000;
        const HOLD_DSPU_OPERATION_W         = 0b0000_0000_0000_0100;
        const INCREMENTAL_INTERFACE_MODE_W  = 0b0000_0000_0000_0011;

    }
}

pub trait Mode1RegisterHandler<SPI, DELAY, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
    DEVICE: DeviceTrait<SPI, DELAY>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DELAY, DEVICE> Mode1RegisterHandler<SPI, DELAY, DEVICE> for Mode1Register
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
    DEVICE: DeviceTrait<SPI, DELAY>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error> {
        todo!()
    }

    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error> {
        todo!()
    }
}

impl RegisterFromRaw for Mode1RegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::Mode1Register;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct Mode1RegisterStructure {
    incremental_interface_mode: IncrementalInterfaceMode,
    hold_dspu_operation: HoldDspuOperation,
    reserved_3: u1,
    clock_source_select: ClockSourceSelect,
    reserved_13_5: u9,
    update_rate_setting: UpdateRateSetting,
}

#[bitsize(2)]
#[derive(FromBits)]
pub enum IncrementalInterfaceMode {
    Disabled = 0,
    AbOperationWithIndexOnIfcPin,
    StepDirectionOperationWithIndexOnIfcPin,
    NotAllowed,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum HoldDspuOperation {
    NormalScheduleOperation = 0,
    DspuOnHold,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ClockSourceSelect {
    InternalOscillator = 0,
    External4MhzClock,
}

#[bitsize(2)]
#[derive(FromBits)]
pub enum UpdateRateSetting {
    Reserved = 0,
    Us42_7,
    Us85_3,
    Us170_6,
}
