#![no_std]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod comproc;
mod function;
/// Main device implementation.
pub mod max17048;
#[allow(missing_docs)] // Field semantics are documented in REGISTERS.md.
pub mod register;
/// Compatibility register address constants.
pub mod register_access;
/// Device errors.
pub mod types;

pub use max17048::{ADDR, MAX17048};
pub use types::Error;

/// Idiomatic spelling of the device name.
pub type Max17048<I2C> = MAX17048<I2C>;
