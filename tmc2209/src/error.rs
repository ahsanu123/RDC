//! Specific device/protocol failures and lossless UART error wrapping.

/// Operation whose nonblocking polling budget expired.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UartOperation {
    Transmit,
    Receive,
    Flush,
}

/// Device errors; the original HAL UART error is never reduced to an error kind.
#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceError<InterfaceError> {
    Interface(InterfaceError),
    InvalidCommunicationSettings,
    Timeout { operation: UartOperation },
    EchoMismatch { expected: u8, actual: u8 },
    InvalidReplySync { actual: u8 },
    InvalidReplyAddress { actual: u8 },
    UnexpectedRegister { expected: u8, actual: u8 },
    CrcMismatch { expected: u8, actual: u8 },
    UnsupportedVersion { actual: u8 },
    WriteNotAcknowledged { expected: u8, actual: u8 },
    InvalidWriteBits { register: u8, bits: u32 },
    TestModeEnabled,
    InvalidOffTime,
    InvalidHysteresis,
    InvalidMicrostepResolution { actual: u8 },
    PinControlledMicrosteps,
    IncompatibleStepFiltering,
    InvalidVelocity { value: i32 },
    InvalidSenseResistance,
    UnsupportedCurrentEstimation,
    StepIntervalUnavailable,
    InvalidOtpByte { actual: u8 },
    OtpBitAlreadyProgrammed,
    OtpVerificationFailed { byte: u8, bit: u8 },
    RecoveryLimitExceeded { discarded: u32 },
}

impl<E> DeviceError<E> {
    pub(crate) fn reported(self) -> Self {
        #[cfg(feature = "defmt")]
        defmt::error!("TMC2209 operation failed; error returned to caller");
        self
    }
}

impl<E> From<E> for DeviceError<E> {
    fn from(error: E) -> Self {
        Self::Interface(error).reported()
    }
}
