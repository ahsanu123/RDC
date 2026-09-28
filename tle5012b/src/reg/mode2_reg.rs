use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// ANG_RANGE 14:4 w Angle Range1)
//     Changes the representation of the angle output (AVAL
//     and ASPD register) by multiplying the output with a factor
//     ANG_RANGE/128.
//     080H factor 1 (default), magnetic angle -180°..180°
//     mapped to values -16384..16383
//     200H factor 4, magnetic angle -45°..45° mapped to
//     values -16384..16383. Values outside this range
//     are clamped to the limit value and S_OV flag is set.
//     040H factor 0.5, magnetic angle -180°..180° mapped to
//     values -8192..8191)
//     Reset: 080H
// ANG_DIR 3 w Angle Direction
//     Inverts angle and angle speed values and revolution
//     counter behaviour.
//     Note: In case of changing ANG_DIR, AUTOCAL should
//     be deactivated as explained under Note on
//     Page 23.
//     0B counterclockwise rotation of magnet
//     1B clockwise rotation of magnet
//     Reset: 0B
// PREDICT 2 w Prediction
//     Prediction of angle value based on current angle speed
//     (see data sheet).
//     Note: In case of changing a PREDICT, AUTOCAL should
//     be deactivated as explained under Note on
//     Page 23.
//     0B prediction disabled
//     1B prediction enabled
//     Reset: derivate-specific
// AUTOCAL 1:0 w Autocalibration Mode
//     Automatic calibration of offset and amplitude
//     synchronicity for applications with full-turn. Only 1 LSB
//     corrected at each update. CRC check of calibration
//     registers is automatically disabled if AUTOCAL activated.
//     Autocalibration is described in the data sheet. Also see
//     Chapter 4.1.
//     00B no auto-calibration
//     01B auto-cal. mode 1: update every angle update cycle
//     (FIR_MD setting)
//     10B auto-cal. mode 2: update every 1.5 revolutions
//     11B auto-cal. mode 3: update every 11.25°
//     Reset: derivate-specific

bitflags! {
    // address = 08H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct Mode2Register: u16{
                                      //FEDC_BA98_7654_3210
        const ANGLE_RANGE_W           = 0b0111_1111_1111_0000;
        const ANGLE_DIRECTION_W       = 0b0000_0000_0000_1000;
        const PREDICTION_W            = 0b0000_0000_0000_0100;
        const AUTOCALIBRATION_MODE_W  = 0b0000_0000_0000_0011;
    }
}

pub trait Mode2RegisterHandler<SPI, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DEVICE: DeviceTrait<SPI>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DEVICE> Mode2RegisterHandler<SPI, DEVICE> for Mode2Register
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

impl RegisterFromRaw for Mode2RegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::Mode2Register;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct Mode2RegisterStructure {
    autocalibration_mode: AutocalibrationMode,
    prediction: Prediction,
    angle_direction: AngleDirection,
    angle_range: u11,
    reserved_15: u1,
}

#[bitsize(2)]
#[derive(FromBits)]
pub enum AutocalibrationMode {
    NoAutoCalibration = 0,
    Mode1UpdateEveryAngleUpdateCycle,
    Mode2UpdateEveryOnePointFiveRevolutions,
    Mode3UpdateEveryElevenPointTwentyFiveDegrees,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum Prediction {
    Disabled = 0,
    Enabled,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum AngleDirection {
    CounterclockwiseRotationOfMagnet = 0,
    ClockwiseRotationOfMagnet,
}
