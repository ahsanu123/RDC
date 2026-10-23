#![no_std]
#![doc = include_str!("../README.md")]

pub mod builder;
pub mod comproc;
mod device;
pub mod error;
mod function;
pub mod register;

pub use bilge::prelude::u3;
pub use device::{MCP23S17, MCP23017};
pub use error::DeviceError;
pub use register::{Direction, Pin, Port};

/// Short alias for [`DeviceError`].
pub type Error<InterfaceError> = DeviceError<InterfaceError>;
