//! Typed BANK=0 registers. Bit zero always corresponds to pin zero.

use bilge::prelude::*;

pub mod defval;
pub mod gpinten;
pub mod gpio;
pub mod gppu;
pub mod intcap;
pub mod intcon;
pub mod intf;
pub mod iocon;
pub mod iodir;
pub mod ipol;
pub mod olat;

pub use defval::Defval;
pub use gpinten::Gpinten;
pub use gpio::Gpio;
pub use gppu::Gppu;
pub use intcap::Intcap;
pub use intcon::Intcon;
pub use intf::Intf;
pub use iocon::Iocon;
pub use iodir::{Direction, Iodir};
pub use ipol::Ipol;
pub use olat::Olat;

/// One of the two eight-bit ports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Port {
    A,
    B,
}

/// A valid pin index within a port.
#[bitsize(3)]
#[derive(Clone, Copy, Debug, FromBits, PartialEq, Eq)]
pub enum Pin {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
}

impl Pin {
    pub(crate) fn mask(self) -> u8 {
        1 << u3::from(self).value()
    }
}

mod sealed {
    pub trait Sealed {}
}

/// A readable register. Implemented only by this crate's typed registers.
pub trait Register: sealed::Sealed + From<u8> + Into<u8> + Copy {
    const ADDRESS_A: u8;
    const RESET: u8;

    fn address(port: Port) -> u8 {
        Self::ADDRESS_A + u8::from(port == Port::B)
    }
}

/// A writable register, excluding IOCON which requires invariant checks.
///
/// ```compile_fail
/// use mcp23017::register::{Intf, WritableRegister};
/// fn writable<R: WritableRegister>() {}
/// writable::<Intf>();
/// ```
pub trait WritableRegister: Register {}

macro_rules! register {
    ($name:ty, $address:expr, $reset:expr) => {
        impl super::sealed::Sealed for $name {}
        impl super::Register for $name {
            const ADDRESS_A: u8 = $address;
            const RESET: u8 = $reset;
        }
        impl Default for $name {
            fn default() -> Self {
                Self::from($reset)
            }
        }
    };
}

pub(crate) use register;
