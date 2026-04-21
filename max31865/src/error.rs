use bitflags::bitflags;

bitflags! {
    #[derive(PartialEq, Eq)]
    pub struct FaultRegister: u8 {
        const OVER_OR_UNDERVOLTAGE = 2;
        const RTDIN_FORCE_MINUS_OPEN = 3;
        const REFIN_FORCE_MINUS_OPEN = 4;
        const REFIN_GT_085_VBIAS = 5;
        const RTD_LOW_THRESHOLD = 6;
        const RTD_HIGH_THRESHOLD = 7;
    }
}

#[derive(Debug)]
pub enum HardwareErr {
    OverOrUnderVoltage,
    RtdinForceMinusOpen,
    RefinForceMinusOpen,
    RefinGreatherThanPoint85TimeVBias,
    RtdLowThreshold,
    RtdHighThreshold,
}

#[derive(Debug)]
pub enum SpiErr {
    FailToRead,
    FailToWrite,
}

#[derive(Debug)]
pub enum Max31865Err {
    Spi(SpiErr),
    Hardware(HardwareErr),
    Operation(&'static str),
}
