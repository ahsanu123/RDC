use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// IrqCtl, 0xFA (CST816S register declaration, pages 4-5).
///
/// Uses the field descriptions: EnMotion is bit 4 and OnceWLP is bit 0.
/// The page 4 drawing conflicts with these assignments; see the README.
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct InterruptControl {
    pub single_pulse_on_long_press: bool,
    pub unspecified: u3,
    pub gesture_interrupt_enabled: bool,
    pub touch_change_interrupt_enabled: bool,
    pub touch_interrupt_enabled: bool,
    pub interrupt_test_enabled: bool,
}

impl InterruptControl {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::InterruptControl;
}

#[cfg(feature = "defmt")]
impl defmt::Format for InterruptControl {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "InterruptControl({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_interrupt_control<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<InterruptControl, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, InterruptControl::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_interrupt_control<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: InterruptControl,
    ) -> Result<(), Cst816sError<I::Error>> {
        let current = comproc::read(i2c, device_address, InterruptControl::ADDRESS)?;
        let updated = (current & !0xF1) | (u8::from(value) & 0xF1);
        comproc::write(i2c, device_address, InterruptControl::ADDRESS, updated)
    }
}
