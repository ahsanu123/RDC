use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// TCO_X_T 15:9 w Offset Temperature Coefficient for X-Component
//     7-bit signed integer value of X-offset temperature
//     coefficient. This register is used both with autocalibration
//     and without autocalibration. If autocalibration is
//     deactivated, overwrite only with the default value. If
//     autocalibration is activated, do not write this bitfield. See
//     “Offset temperature compensation” on Page 97.
//     Reset: device-specific
// HSM_PLP (multi-purpose) 8:5 w Hall Switch Mode: Pole-Pair Configuration
//     0000B 1 pole pairs
//     0001B 2 pole pairs
//     0010B 3 pole pairs
//     ...B ...
//     1101B 14 pole pairs
//     1110B 15 pole pairs
//     1111B 16 pole pairs
//     Pulse-Width-Modulation Mode: Error Indication
//     xx0xB error indication enabled
//     xx1xB error indication disabled
//     Incremental Interface Mode: Absolute Count
//     Interface counts to absolute value at startup
//     x0xxB absolute count enabled
//     x1xxB absolute count disabled
//     SPC Mode: Total Trigger Time
//     Duration of the master pulse to trigger SPC output
//     0000B 90*UT
//     0100B tmlow + 12 UT
//     Reset: derivate-specific
// IFAB_RES (multi-purpose) 4:3 w Pulse-Width-Modulation Mode: Frequency
//     Selection of PWM frequency.
//     00B 244 Hz
//     01B 488 Hz
//     10B 977 Hz
//     11B 1953 Hz
//     Incremental Interface Mode: IIF resolution
//     00B 12bit, 0.088° step
//     01B 11bit, 0.176° step
//     10B 10bit, 0.352° step
//     11B 9bit, 0.703° step
//     SPC Mode: SPC Frame Configuration
//     00B 12bit angle
//     01B 16bit angle
//     10B 12bit angle + 8bit temperature
//     11B 16bit angle + 8bit temperature
//     Reset: derivate-specific
// IF_MD 1:0 w Interface Mode on IFA,IFB,IFC
//     Any derivate can be configurated to operate in any of the
//     four following protocols on the IFA, IFB and IFC outputs.
//     Reconfiguration is required at every start-up, else the
//     default protocol of the derivate will be used.
//     SSC interface is always active in parallel on pins SCK,
//     CSQ and DATA.
//     00B IIF
//     01B PWM
//     10B HSM
//     11B SPC1)
//     Reset: derivate-specific

bitflags! {
    // address = 0EH
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct Mode4Register: u16{
                                                                //FEDC_BA98_7654_3210
        const OFFSET_TEMPERATURE_COEFFICIENT_FOR_X_COMPONENT  = 0b1111_1110_0000_0000;
        const HALL_SWITCH_MODE                                = 0b0000_0001_1110_0000;
        const PULSE_WIDTH_MODULATION_MODE                     = 0b0000_0000_0001_1000;
        const INTERFACE_MODE_ON_IFA_IFB_IFC                   = 0b0000_0000_0000_0011;
    }
}

pub trait Mode4RegisterHandler<SPI, DELAY, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
    DEVICE: DeviceTrait<SPI, DELAY>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DELAY, DEVICE> Mode4RegisterHandler<SPI, DELAY, DEVICE> for Mode4Register
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

impl RegisterFromRaw for Mode4RegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::Mode4Register;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct Mode4RegisterStructure {
    interface_mode_on_ifa_ifb_ifc: InterfaceModeOnIFAIFBIFC,
    reserved_2: u1,
    ifab_resolution: IFABResolution,
    hall_switch_mode_config: HallSwitchModePolePairConfiguration,
    offset_temperature_coefficient_for_x_component: u7,
}

#[bitsize(2)]
#[derive(FromBits)]
pub enum InterfaceModeOnIFAIFBIFC {
    IIF = 0,
    PWM,
    HSM,
    SPC,
}

#[bitsize(2)]
#[derive(FromBits)]
pub enum IFABResolution {
    Pwm244HzOrIif12BitOrSpc12BitAngle = 0,
    Pwm488HzOrIif11BitOrSpc16BitAngle,
    Pwm977HzOrIif10BitOrSpc12BitAngleAnd8BitTemperature,
    Pwm1953HzOrIif9BitOrSpc16BitAngleAnd8BitTemperature,
}

#[bitsize(4)]
#[derive(FromBits)]
pub enum HallSwitchModePolePairConfiguration {
    OnePolePair = 0,
    TwoPolePairs,
    ThreePolePairs,
    FourPolePairs,
    FivePolePairs,
    SixPolePairs,
    SevenPolePairs,
    EightPolePairs,
    NinePolePairs,
    TenPolePairs,
    ElevenPolePairs,
    TwelvePolePairs,
    ThirteenPolePairs,
    FourteenPolePairs,
    FifteenPolePairs,
    SixteenPolePairs,
}
