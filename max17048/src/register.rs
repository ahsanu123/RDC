//! Typed register words; see REGISTERS.md for access and reset semantics.

mod cell_voltage;
mod charge_rate;
mod command;
mod configuration;
mod hibernate_threshold;
mod mode;
mod model;
mod reset_voltage;
mod state_of_charge;
mod status;
mod version;
mod voltage_alert;

pub use cell_voltage::CellVoltage;
pub use charge_rate::ChargeRate;
pub use command::Command;
pub use configuration::Configuration;
pub use hibernate_threshold::HibernateThreshold;
pub use mode::Mode;
pub use model::{ModelAccess, ModelTableWord};
pub use reset_voltage::ResetVoltage;
pub use state_of_charge::StateOfCharge;
pub use status::Status;
pub use version::Version;
pub use voltage_alert::VoltageAlert;

/// A readable, fixed-address register. Reserved addresses are not exposed.
pub trait ReadableRegister: From<u16> {
    /// Address of the most significant byte.
    const ADDRESS: u8;
}

macro_rules! readable {
    ($name:ident, $address:expr) => {
        impl super::ReadableRegister for $name {
            const ADDRESS: u8 = $address;
        }
    };
}
pub(crate) use readable;
