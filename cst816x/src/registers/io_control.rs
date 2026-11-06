use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// En1v8 voltage selection (CST816S register declaration, page 5).
#[bitsize(1)]
#[derive(FromBits, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IoVoltage {
    Vdd = 0,
    Volts1_8 = 1,
}

/// IIC_OD pin driver selection (CST816S register declaration, page 5).
#[bitsize(1)]
#[derive(FromBits, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TwoWireDriverMode {
    PullUpResistor = 0,
    OpenDrain = 1,
}

/// IOCtl, 0xFD (CST816S register declaration, page 5).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct IoControl {
    pub voltage: IoVoltage,
    pub two_wire_driver_mode: TwoWireDriverMode,
    pub irq_soft_reset_enabled: bool,
    pub unspecified: u5,
}

impl IoControl {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::IoControl;
}

#[cfg(feature = "defmt")]
impl defmt::Format for IoControl {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "IoControl({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_io_control<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<IoControl, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, IoControl::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_io_control<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: IoControl,
    ) -> Result<(), Cst816sError<I::Error>> {
        let current = comproc::read(i2c, device_address, IoControl::ADDRESS)?;
        let updated = (current & !0x07) | (u8::from(value) & 0x07);
        comproc::write(i2c, device_address, IoControl::ADDRESS, updated)
    }
}
