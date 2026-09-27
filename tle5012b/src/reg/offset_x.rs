use crate::device::DeviceTrait;
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// X_OFFSET 15:4 w Offset Correction of X-value in digits
//     12-bit signed integer value of raw X-signal offset
//     correction at 25°C.
//     Reset: device-specific

bitflags! {
    // address = 0AH
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct OffsetXRegister: u16{
                                                         //FEDC_BA98_7654_3210
        const OFFSET_CORRECTION_OF_X_VALUE_IN_DIGITS_W = 0b1111_1111_1111_0000;
    }
}

pub trait OffsetXRegisterHandler<SPI, DELAY, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
    DEVICE: DeviceTrait<SPI, DELAY>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DELAY, DEVICE> OffsetXRegisterHandler<SPI, DELAY, DEVICE> for OffsetXRegister
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

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct OffsetXRegisterStructure {
    reserved_3_0: u4,
    correction_value_in_digits: u12,
}
