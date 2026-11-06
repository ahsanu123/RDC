use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// IrqPluseWidth, 0xED (CST816S register declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct InterruptPulseWidth {
    /// Raw 0.1 ms units; documented range: 1-200.
    pub tenths_of_millisecond: u8,
}

impl Default for InterruptPulseWidth {
    fn default() -> Self {
        Self::new(10)
    }
}

impl InterruptPulseWidth {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::InterruptPulseWidth;
}

#[cfg(feature = "defmt")]
impl defmt::Format for InterruptPulseWidth {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "InterruptPulseWidth({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_interrupt_pulse_width<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<InterruptPulseWidth, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, InterruptPulseWidth::ADDRESS).map(Into::into)
    }

    pub(crate) fn write_interrupt_pulse_width<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
        value: InterruptPulseWidth,
    ) -> Result<(), Cst816sError<I::Error>> {
        comproc::write(
            i2c,
            device_address,
            InterruptPulseWidth::ADDRESS,
            value.into(),
        )
    }
}
