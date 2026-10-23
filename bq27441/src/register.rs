//! BQ27441-G1 standard and extended commands (SLUUAC9A chapters 4 and 5).

pub mod average_current;
pub mod average_power;
pub mod block_data;
pub mod block_data_checksum;
pub mod block_data_control;
pub mod control;
pub mod data_block;
pub mod data_class;
pub mod design_capacity;
pub mod flags;
pub mod full_available_capacity;
pub mod full_charge_capacity;
pub mod full_charge_capacity_filtered;
pub mod full_charge_capacity_unfiltered;
pub mod internal_temperature;
pub mod maximum_load_current;
pub mod nominal_available_capacity;
pub mod operation_configuration;
pub mod remaining_capacity;
pub mod remaining_capacity_filtered;
pub mod remaining_capacity_unfiltered;
pub mod standby_current;
pub mod state_of_charge;
pub mod state_of_charge_unfiltered;
pub mod state_of_health;
pub mod temperature;
pub mod voltage;

pub use average_current::AverageCurrent;
pub use average_power::AveragePower;
pub use block_data::BlockData;
pub use block_data_checksum::BlockDataChecksum;
pub use block_data_control::BlockDataControl;
pub use control::{Control, ControlCommand, ControlStatus};
pub use data_block::DataBlock;
pub use data_class::{DataClass, DataSubclass};
pub use design_capacity::DesignCapacity;
pub use flags::Flags;
pub use full_available_capacity::FullAvailableCapacity;
pub use full_charge_capacity::FullChargeCapacity;
pub use full_charge_capacity_filtered::FullChargeCapacityFiltered;
pub use full_charge_capacity_unfiltered::FullChargeCapacityUnfiltered;
pub use internal_temperature::InternalTemperature;
pub use maximum_load_current::MaximumLoadCurrent;
pub use nominal_available_capacity::NominalAvailableCapacity;
pub use operation_configuration::OperationConfiguration;
pub use remaining_capacity::RemainingCapacity;
pub use remaining_capacity_filtered::RemainingCapacityFiltered;
pub use remaining_capacity_unfiltered::RemainingCapacityUnfiltered;
pub use standby_current::StandbyCurrent;
pub use state_of_charge::StateOfCharge;
pub use state_of_charge_unfiltered::StateOfChargeUnfiltered;
pub use state_of_health::{HealthStatus, StateOfHealth};
pub use temperature::Temperature;
pub use voltage::Voltage;

mod sealed {
    pub trait Sealed {}
}

/// A readable two-byte command. Words are little-endian on the I2C bus.
/// Control responses must instead be selected using `read_control`.
pub trait Register: sealed::Sealed + From<u16> + Into<u16> + Copy {
    const ADDRESS: u8;
    /// `None` means dynamic or unspecified, not zero.
    const RESET: Option<u16>;
}

macro_rules! register {
    ($name:ty, $address:expr, $reset:expr) => {
        impl super::sealed::Sealed for $name {}
        impl super::Register for $name {
            const ADDRESS: u8 = $address;
            const RESET: Option<u16> = $reset;
        }
    };
}

pub(crate) use register;
