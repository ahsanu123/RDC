//! Device-specific failures and lossless interface-error wrapping.

/// Errors returned by MCP23017 and MCP23S17 device operations.
///
/// Configuration failures refer to DS20001952D section 3.5.6; the input-pin
/// restriction is listed in the MCP23017 features.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceError<InterfaceError> {
    /// The original I2C or SPI error, preserved without conversion to an error kind.
    Interface(InterfaceError),
    /// BANK=1 would change the register addresses expected by the driver.
    UnsupportedRegisterBank,
    /// The unimplemented configuration bit must remain zero.
    ReservedConfigurationBitSet,
    /// SPI hardware addressing must remain enabled.
    HardwareAddressingDisabled,
    /// MCP23017 GPA7 and GPB7 support output only.
    UnsupportedInputPin,
}

impl<InterfaceError> From<InterfaceError> for DeviceError<InterfaceError> {
    fn from(error: InterfaceError) -> Self {
        #[cfg(feature = "defmt")]
        defmt::error!("MCP23x17 interface transaction failed");
        Self::Interface(error)
    }
}
