//! Typed register map, MEMSIC MMC5983MA Rev A, pages 13-16.

mod control0;
mod control1;
mod control2;
mod control3;
mod output;
mod status;

pub use control0::*;
pub use control1::*;
pub use control2::*;
pub use control3::*;
pub use output::*;
pub use status::*;

/// Fixed seven-bit I2C address (Rev A, page 17).
pub const I2C_ADDRESS: u8 = 0x30;
/// Expected product identification (Rev A, page 16).
pub const PRODUCT_ID: u8 = 0x30;

/// Register addresses from Rev A, page 13.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Address {
    XOutputHigh = 0x00,
    XOutputLow = 0x01,
    YOutputHigh = 0x02,
    YOutputLow = 0x03,
    ZOutputHigh = 0x04,
    ZOutputLow = 0x05,
    OutputLeastSignificantBits = 0x06,
    Temperature = 0x07,
    Status = 0x08,
    Control0 = 0x09,
    Control1 = 0x0a,
    Control2 = 0x0b,
    Control3 = 0x0c,
    ProductIdentification = 0x2f,
}
