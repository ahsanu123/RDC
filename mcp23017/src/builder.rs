//! Type-checked transport selection with `typed-builder`.

use typed_builder::TypedBuilder;

use crate::{DeviceError, comproc::Interface, device::MCP23017};

/// Build a connection, then initialize it with [`Self::connect`].
///
/// The interface type selects I2C or SPI at compile time; each interface owns
/// its bus and three-bit hardware address. See the README for an example.
/// Datasheet: DS20001952D, sections 3.2-3.3, Serial Interface and Hardware Address.
#[derive(TypedBuilder)]
pub struct Builder<I> {
    interface: I,
}

impl<I: Interface> Builder<I> {
    /// Connect after hardware reset/power-on. SPI startup enables addressing
    /// on all reset MCP23S17s sharing this device's CS.
    pub fn connect(self) -> Result<MCP23017<I>, DeviceError<I::Error>> {
        MCP23017::connect(self.interface)
    }
}
