#![no_std]
#![doc = include_str!("../README.md")]

pub mod builder;
mod comproc;
pub mod config;
pub mod error;
mod function;
pub mod register;
pub mod tmc2209;
pub mod validation;

pub use bilge::prelude::{u2, u3, u4, u5, u20, u24};
pub use config::{CommunicationSettings, EchoMode};
pub use error::{DeviceError, UartOperation};
pub use register::{MicrostepResolution, StandstillMode};
pub use tmc2209::TMC2209;
