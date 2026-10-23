/// Invalid or unsupported hardware configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ConfigurationError {
    ReservedBits,
    CommandBits,
    InvalidChannelInhibit,
    InvalidContinuousFrequency,
    InsufficientBandwidth,
    PeriodicSetRequiresAutomaticAndContinuous,
    ThreeWireSpiUnsupported,
    OpposingSelfTestCurrents,
}

/// Device errors never discard the underlying transport error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<InterfaceError> {
    Interface(InterfaceError),
    NotInitialized,
    UnexpectedProductId(u8),
    OtpNotReady,
    InvalidConfiguration(ConfigurationError),
    ContinuousModeActive,
    AutomaticSetResetActive,
    ChannelsDisabled,
    MeasurementNotReady,
    MeasurementTimeout,
}
