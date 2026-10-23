//! Device failures and lossless HAL error wrapping.

/// Errors returned by BQ27441 operations.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceError<InterfaceError> {
    Interface(InterfaceError),
    UnexpectedDevice {
        device_type: u16,
        data_memory_code: u16,
        chemistry_identifier: u16,
    },
    InvalidControlQuery,
    InvalidControlAction,
    Sealed,
    ShutdownNotEnabled,
    ConfigurationUpdateRequired,
    ConfigurationUpdateTimeout {
        expected: bool,
    },
    SealStateTimeout {
        expected: bool,
    },
    ExternalTemperatureDisabled,
    InvalidDesignCapacity,
    InvalidDesignEnergy,
    InvalidTerminateVoltage,
    InvalidTaperRate,
    InvalidDataBlock,
    ReadOnlyDataMemory,
    ChecksumMismatch {
        expected: u8,
        received: u8,
    },
    BlockVerificationFailed,
}

impl<E> From<E> for DeviceError<E> {
    fn from(error: E) -> Self {
        #[cfg(feature = "defmt")]
        defmt::error!("BQ27441 I2C transaction failed");
        Self::Interface(error)
    }
}

impl<E> DeviceError<E> {
    pub(crate) fn reported(self) -> Self {
        #[cfg(feature = "defmt")]
        defmt::warn!("BQ27441 operation rejected or verification failed");
        self
    }
}
