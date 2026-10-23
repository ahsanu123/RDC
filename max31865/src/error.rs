use bitflags::bitflags;

use crate::register::FaultStatus;

bitflags! {
    /// Compatibility fault masks; prefer the typed FaultStatus register.
    #[derive(PartialEq, Eq)]
    pub struct FaultRegister: u8 {
        const OVER_OR_UNDERVOLTAGE = 1 << 2;
        const RTDIN_FORCE_MINUS_OPEN = 1 << 3;
        const REFIN_FORCE_MINUS_OPEN = 1 << 4;
        const REFIN_GT_085_VBIAS = 1 << 5;
        const RTD_LOW_THRESHOLD = 1 << 6;
        const RTD_HIGH_THRESHOLD = 1 << 7;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HardwareErr {
    OverOrUnderVoltage,
    RtdinForceMinusOpen,
    RefinForceMinusOpen,
    RefinGreatherThanPoint85TimeVBias,
    RtdLowThreshold,
    RtdHighThreshold,
    MultipleFaults(FaultStatus),
    UnspecifiedRtdFault,
}

/// Legacy operation-only SPI errors, retained for source compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SpiErr {
    FailToRead,
    FailToWrite,
}

/// Driver errors retain the original interface error in Interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Max31865Err<InterfaceError = core::convert::Infallible> {
    Interface(InterfaceError),
    Spi(SpiErr),
    Hardware(HardwareErr),
    Operation(&'static str),
    InvalidNominalResistance,
    InvalidReferenceResistance,
    InvalidConversion,
}
