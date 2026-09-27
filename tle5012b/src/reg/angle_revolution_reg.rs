use crate::{
    communication::possible_address::PossibleAddress, device::DeviceTrait,
    reg::prelude::RegisterFromRaw,
};
use bilge::prelude::*;
use bitflags::bitflags;

// Field Bits Type Description
//
// RD_REV 15 r Read Status, Revolution
//     0B no new values since last readout
//     1B new value (REVOL) present. The bit is cleared on
//     a read-out (volid for both: normal operation and
//     update buffer). Note: If an update event (register
//     snapshot) is done after a normal read, RD_REV will
//     not be set to 1B in the following read (either update
//     read or normal read) unless a new value is
//     available.
//     Reset: 1B
// FCNT 14:9 wu Frame Counter (unsigned 6-bit value)
//     Internal frame counter. Increments every update period
//     (FIR_MD setting).
//     Reset: 0H
// REVOL 8:0 ru Number of Revolutions (signed 9-bit value)
//     Revolution counter. Increments for every full rotation in
//     counter-clockwise direction (at angle discontinuity from
//     360° to 0°) and decrements for every full rotation in
//     clockwise direction (at angle discontinuity from 0° to
//     360°). Also see Chapter 4.2.
//     Reset: 0H

bitflags! {
    // address = 04H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    #[derive(Default)]
    pub struct AngleRevolutionRegister: u16{
                                                  //FEDC_BA98_7654_3210
        const READ_STATUS_REVOLUTION            = 0b1000_0000_0000_0000;
        const FRAME_COUNTER_U6BIT_WU            = 0b0111_1110_0000_0000;
        const NUMBER_OF_REVOLUTION_SBIT_SRU     = 0b0000_0001_1111_1111;
    }
}

pub trait AngleRevolutionRegisterHandler<SPI, DELAY, DEVICE>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
    DEVICE: DeviceTrait<SPI, DELAY>,
{
    fn read_status(&mut self, dev: &mut DEVICE) -> Result<(), SPI::Error>;
    fn write_slave_number(&mut self, dev: &mut DEVICE, number: u2) -> Result<(), SPI::Error>;
}

impl<SPI, DELAY, DEVICE> AngleRevolutionRegisterHandler<SPI, DELAY, DEVICE>
    for AngleRevolutionRegister
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

impl RegisterFromRaw for AngleRevolutionRegisterStructure {
    const ADDRESS: PossibleAddress = PossibleAddress::AngleRevolutionRegister;

    fn into_u16(self) -> u16 {
        self.value
    }
}

#[bitsize(16)]
#[derive(FromBits)]
// LSB field write first
pub struct AngleRevolutionRegisterStructure {
    number_of_revolutions: u9,
    frame_counter: u6,
    read_status_revolution: ReadStatusRevolution,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ReadStatusRevolution {
    NoNewValuesSinceLastReadout = 0,
    NewValuePresent,
}
