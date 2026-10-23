//! MAX31865 Rev 3, pages 13-16: typed registers and compatibility constants.

mod configuration;
mod fault_status;
mod legacy;
mod resistance;
mod threshold;

pub use configuration::{Configuration, MainsFilter};
pub use fault_status::FaultStatus;
pub use legacy::Register;
pub use resistance::RtdResistance;
pub use threshold::FaultThreshold;

/// Read addresses from MAX31865 Rev 3, Table 1, page 13.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Address {
    Configuration = 0x00,
    RtdResistanceMostSignificant = 0x01,
    RtdResistanceLeastSignificant = 0x02,
    HighFaultThresholdMostSignificant = 0x03,
    HighFaultThresholdLeastSignificant = 0x04,
    LowFaultThresholdMostSignificant = 0x05,
    LowFaultThresholdLeastSignificant = 0x06,
    FaultStatus = 0x07,
}

impl From<Address> for Register {
    fn from(address: Address) -> Self {
        Self::from_bits_retain(address as u8)
    }
}
