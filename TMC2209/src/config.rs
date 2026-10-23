use typed_builder::TypedBuilder;

use crate::{DeviceError, u2};

/// Whether UART RX receives a byte-for-byte copy of TX on the single-wire connection.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EchoMode {
    #[default]
    Enabled,
    Disabled,
}

/// UART topology, timing budgets and device clock, not a UART peripheral configurator.
/// Datasheet rev. 1.09, sections 4.1-4.3 and 18 (clock).
#[derive(Clone, Copy, Debug, TypedBuilder)]
pub struct CommunicationSettings {
    #[builder(default = u2::new(0))]
    pub address: u2,
    #[builder(default)]
    pub echo: EchoMode,
    #[builder(default = 115_200)]
    pub baud_rate: u32,
    #[builder(default = 10_000)]
    pub polling_attempts: u32,
    #[builder(default = 10)]
    pub polling_delay_us: u32,
    #[builder(default = 12_000_000)]
    pub clock_hz: u32,
}

impl Default for CommunicationSettings {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl CommunicationSettings {
    pub(crate) fn validate<E>(&self) -> Result<(), DeviceError<E>> {
        if self.baud_rate < 9_000
            || u64::from(self.baud_rate) * 16 > u64::from(self.clock_hz)
            || self.polling_attempts == 0
            || self.polling_delay_us == 0
        {
            return Err(DeviceError::InvalidCommunicationSettings.reported());
        }
        Ok(())
    }
}
