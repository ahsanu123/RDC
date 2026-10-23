use bilge::prelude::*;

/// Latched Fault Status 07h, cleared through configuration D1 (Rev 3, Table 7).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Default, Clone, Copy, PartialEq, Eq)]
pub struct FaultStatus {
    pub ignored: u2,
    pub over_or_under_voltage: bool,
    pub rtdin_low_with_force_open: bool,
    pub refin_low_with_force_open: bool,
    pub refin_above_bias_threshold: bool,
    pub rtd_low_threshold: bool,
    pub rtd_high_threshold: bool,
}

#[cfg(feature = "defmt")]
impl defmt::Format for FaultStatus {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(formatter, "FaultStatus({=u8:#x})", u8::from(*self));
    }
}
