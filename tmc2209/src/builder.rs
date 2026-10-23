use embedded_hal::delay::DelayNs;
use embedded_hal_nb::serial::{Read, Write};
use typed_builder::TypedBuilder;

use crate::{DeviceError, TMC2209, config::CommunicationSettings};

/// Type-checked UART, delay and connection configuration.
/// Datasheet rev. 1.09, section 4; selects echo topology, not another chip protocol.
#[derive(TypedBuilder)]
pub struct Builder<UART, DELAY> {
    uart: UART,
    delay: DELAY,
    #[builder(default)]
    settings: CommunicationSettings,
}

impl<UART: Read<u8> + Write<u8>, DELAY: DelayNs> Builder<UART, DELAY> {
    pub fn connect(self) -> Result<TMC2209<UART, DELAY>, DeviceError<UART::Error>> {
        TMC2209::new(self.uart, self.delay, self.settings)
    }
}
