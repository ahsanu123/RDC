//! Typed BANK=0 registers. Bit zero always corresponds to pin zero.

use bilge::prelude::*;

pub mod configuration;
pub mod general_purpose_input_output;
pub mod input_direction;
pub mod input_polarity;
pub mod interrupt_capture;
pub mod interrupt_control;
pub mod interrupt_default_value;
pub mod interrupt_enable;
pub mod interrupt_flags;
pub mod output_latch;
pub mod pull_up_resistors;

pub use configuration::Configuration;
pub use general_purpose_input_output::GeneralPurposeInputOutput;
pub use input_direction::{Direction, InputDirection};
pub use input_polarity::InputPolarity;
pub use interrupt_capture::InterruptCapture;
pub use interrupt_control::InterruptControl;
pub use interrupt_default_value::InterruptDefaultValue;
pub use interrupt_enable::InterruptEnable;
pub use interrupt_flags::InterruptFlags;
pub use output_latch::OutputLatch;
pub use pull_up_resistors::PullUpResistors;

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
/// use mcp23017::register::{InterruptFlags, WritableRegister};
/// fn writable<R: WritableRegister>() {}
/// writable::<InterruptFlags>();
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
