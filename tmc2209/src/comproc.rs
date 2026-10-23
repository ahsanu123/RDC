//! Bounded UART framing, echo handling and recovery.

use embedded_hal::delay::DelayNs;
use embedded_hal_nb::serial::{Read, Write};

use crate::{
    DeviceError, UartOperation,
    config::{CommunicationSettings, EchoMode},
    validation::{crc8, validate_reply},
};

const SYNC_BYTE: u8 = 0x05;
const WRITE_FLAG: u8 = 0x80;
const INTER_DATAGRAM_IDLE_BITS: u32 = 16;
const RECOVERY_IDLE_BITS: u32 = 80;

/// Owns the UART and delay provider; serial operations must themselves return promptly.
/// Datasheet rev. 1.09, sections 4.1-4.3 (framing, idle recovery and single-wire echo).
pub(crate) struct UartTransport<UART, DELAY> {
    uart: UART,
    delay: DELAY,
    settings: CommunicationSettings,
}

impl<UART: Read<u8> + Write<u8>, DELAY: DelayNs> UartTransport<UART, DELAY> {
    pub(crate) fn new(
        uart: UART,
        delay: DELAY,
        settings: CommunicationSettings,
    ) -> Result<Self, DeviceError<UART::Error>> {
        settings.validate()?;
        Ok(Self {
            uart,
            delay,
            settings,
        })
    }

    fn poll<T>(
        &mut self,
        operation: UartOperation,
        mut action: impl FnMut(&mut UART) -> nb::Result<T, UART::Error>,
    ) -> Result<T, DeviceError<UART::Error>> {
        for attempt in 0..self.settings.polling_attempts {
            match action(&mut self.uart) {
                Ok(value) => return Ok(value),
                Err(nb::Error::Other(error)) => return Err(error.into()),
                Err(nb::Error::WouldBlock) => {
                    if attempt + 1 < self.settings.polling_attempts {
                        self.delay.delay_us(self.settings.polling_delay_us);
                    }
                }
            }
        }
        Err(DeviceError::Timeout { operation }.reported())
    }

    fn receive_byte(&mut self) -> Result<u8, DeviceError<UART::Error>> {
        self.poll(UartOperation::Receive, Read::read)
    }

    fn send_frame(&mut self, frame: &[u8]) -> Result<(), DeviceError<UART::Error>> {
        for &byte in frame {
            self.poll(UartOperation::Transmit, |uart| uart.write(byte))?;
            if self.settings.echo == EchoMode::Enabled {
                let echo = self.receive_byte()?;
                if echo != byte {
                    return Err(DeviceError::EchoMismatch {
                        expected: byte,
                        actual: echo,
                    }
                    .reported());
                }
            }
        }
        self.poll(UartOperation::Flush, Write::flush)
    }

    fn wait_bit_times(&mut self, bits: u32) {
        let microseconds =
            (u64::from(bits) * 1_000_000).div_ceil(u64::from(self.settings.baud_rate));
        self.delay.delay_us(microseconds as u32);
    }

    pub(crate) fn read(&mut self, register: u8) -> Result<u32, DeviceError<UART::Error>> {
        self.wait_bit_times(INTER_DATAGRAM_IDLE_BITS);
        let mut request = [SYNC_BYTE, self.settings.address.value(), register, 0];
        request[3] = crc8(&request[..3]);
        self.send_frame(&request)?;
        let mut reply = [0; 8];
        for byte in &mut reply {
            *byte = self.receive_byte()?;
        }
        validate_reply(&reply, register)
    }

    pub(crate) fn write(
        &mut self,
        register: u8,
        value: u32,
    ) -> Result<(), DeviceError<UART::Error>> {
        self.wait_bit_times(INTER_DATAGRAM_IDLE_BITS);
        let [a, b, c, d] = value.to_be_bytes();
        let mut request = [
            SYNC_BYTE,
            self.settings.address.value(),
            register | WRITE_FLAG,
            a,
            b,
            c,
            d,
            0,
        ];
        request[7] = crc8(&request[..7]);
        self.send_frame(&request)
    }

    pub(crate) fn recover(&mut self) -> Result<u32, DeviceError<UART::Error>> {
        self.poll(UartOperation::Flush, Write::flush)?;
        self.wait_bit_times(RECOVERY_IDLE_BITS);
        for discarded in 0..self.settings.polling_attempts {
            match self.uart.read() {
                Ok(_) => {}
                Err(nb::Error::WouldBlock) => return Ok(discarded),
                Err(nb::Error::Other(error)) => return Err(error.into()),
            }
        }
        Err(DeviceError::RecoveryLimitExceeded {
            discarded: self.settings.polling_attempts,
        }
        .reported())
    }

    pub(crate) fn delay_programming(&mut self) {
        self.delay.delay_ms(10);
    }

    pub(crate) fn clock_hz(&self) -> u32 {
        self.settings.clock_hz
    }

    pub(crate) fn release(self) -> (UART, DELAY) {
        (self.uart, self.delay)
    }
}
