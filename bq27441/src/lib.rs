#![no_std]
#![doc = include_str!("../README.md")]

mod comproc;
mod device;
mod error;
mod function;
pub mod register;
mod validation;

pub use device::{BQ27441, BatteryConfiguration, Variant};
pub use error::DeviceError;
