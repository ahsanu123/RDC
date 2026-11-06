use bilge::prelude::*;
use embedded_hal::i2c::I2c;

use super::{Address, Register};
use crate::{comproc, error::Cst816sError};

/// ProjID, 0xA8 (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct ProjectIdentifier {
    pub identifier: u8,
}

impl ProjectIdentifier {
    /// Address of this byte register.
    pub const ADDRESS: Address = Address::ProjectIdentifier;
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProjectIdentifier {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "ProjectIdentifier({=u8:#x})", u8::from(*self));
    }
}

impl Register {
    pub(crate) fn read_project_identifier<I: I2c>(
        &self,
        i2c: &mut I,
        device_address: u8,
    ) -> Result<ProjectIdentifier, Cst816sError<I::Error>> {
        comproc::read(i2c, device_address, ProjectIdentifier::ADDRESS).map(Into::into)
    }
}
