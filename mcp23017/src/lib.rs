#![no_std]
#![doc = include_str!("../README.md")]

pub mod builder;
pub mod comproc;
mod device;
mod function;
pub mod register;

pub use bilge::prelude::u3;
pub use device::{Error, MCP23S17, MCP23017};
pub use register::{Direction, Pin, Port};
