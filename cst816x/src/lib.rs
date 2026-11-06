#![no_std]
#![doc = include_str!("../README.md")]

mod comproc;
pub mod device;
pub mod error;
mod function;
pub mod register;

#[allow(deprecated)]
pub use device::Cst816x;
pub use device::{Cst816s, TouchPosition, TouchReading};
pub use error::{Cst816sAddressError, Cst816sError};
/// Compatibility spelling for the typed register module.
pub use register as registers;
