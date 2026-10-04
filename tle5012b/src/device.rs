mod angle_range_writer;
mod angle_speed_reader;
mod angval_reader;
mod config_writer;

pub mod prelude {
    pub use super::angle_range_writer::*;
    pub use super::angle_speed_reader::*;
    pub use super::angval_reader::*;
    pub use super::config_writer::*;
}

use crate::{
    TLE5012B,
    communication::{
        command::{Command, CommandWithWordData},
        possible_address::{PossibleAddress, resulted_register},
        safety_word::SafetyWord,
    },
    crc_table::does_crc_match,
    reg::prelude::RegisterFromRaw,
};
use embedded_hal::spi::{Error as SpiErr, Operation};

#[allow(unused_macros)]
macro_rules! logi {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")] // Or whatever your cfg flag is
        defmt::info!($($arg)*);
    };
}

#[allow(unused_macros)]
macro_rules! loge {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")] // Or whatever your cfg flag is
        defmt::error!($($arg)*);
    };
}

const RW_DELAY: u32 = 180;
const SYSTEM_ERROR_MASK: u16 = 0x4000;
const INTERFACE_ERROR_MASK: u16 = 0x2000;
const INVALID_ANGLE_ERROR_MASK: u16 = 0x1000;

pub struct DeviceInner<SPI>
where
    SPI: embedded_hal::spi::SpiDevice,
{
    spi: SPI,
}

impl<SPI> DeviceInner<SPI>
where
    SPI: embedded_hal::spi::SpiDevice,
{
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }
}

fn inspect_safety<const DATA_LENGTH: usize>(
    message: &[u8; DATA_LENGTH],
    raw_safety_word: u16,
) -> SafetyWord {
    if raw_safety_word & SYSTEM_ERROR_MASK == 0 {
        loge!("system error in safety word");
    }
    if raw_safety_word & INTERFACE_ERROR_MASK == 0 {
        loge!("interface access error in safety word");
    }
    if raw_safety_word & INVALID_ANGLE_ERROR_MASK == 0 {
        loge!("invalid angle in safety word");
    }

    let crc = does_crc_match(message, raw_safety_word);
    if !crc.is_value_match {
        loge!(
            "crc not match, expected {}, got {}",
            crc.expected_value,
            crc.real_value
        );
    }

    SafetyWord::from(raw_safety_word)
}

#[derive(Debug)]
pub enum DeviceError<SPIError: SpiErr> {
    SpiError(SPIError),
    CRCMismatch { expected: u8, got: u8 },
    // ....
    // etc ....
}

pub trait DeviceTrait<SPI>
where
    SPI: embedded_hal::spi::SpiDevice,
{
    fn read(&mut self, address: PossibleAddress) -> Result<u32, DeviceError<SPI::Error>>;

    fn write(
        &mut self,
        address: PossibleAddress,
        data: u16,
    ) -> Result<SafetyWord, DeviceError<SPI::Error>>;

    fn read_then_mutate<T>(
        &mut self,
        mutator_fn: impl FnOnce(&mut T),
    ) -> Result<SafetyWord, DeviceError<SPI::Error>>
    where
        T: RegisterFromRaw;
}

impl<SPI> DeviceTrait<SPI> for DeviceInner<SPI>
where
    SPI: embedded_hal::spi::SpiDevice,
{
    fn read(&mut self, address: PossibleAddress) -> Result<u32, DeviceError<SPI::Error>> {
        let command = Command::read_command(address);
        let raw_command: [u8; 2] = command.value().to_be_bytes();
        let mut buffer: [u8; 4] = [0u8; 4];

        self.spi
            .transaction(&mut [
                Operation::Write(&raw_command),
                Operation::DelayNs(RW_DELAY),
                Operation::Read(&mut buffer),
            ])
            .map_err(DeviceError::SpiError)?;

        let data = u16::from_be_bytes([buffer[0], buffer[1]]);
        let raw_safety_word = u16::from_be_bytes([buffer[2], buffer[3]]);
        let crc_message = [raw_command[0], raw_command[1], buffer[0], buffer[1]];
        inspect_safety(&crc_message, raw_safety_word);

        let result = ((data as u32) << 16) | raw_safety_word as u32;
        logi!("read result{}", result);
        Ok(result)
    }

    fn write(
        &mut self,
        address: PossibleAddress,
        data: u16,
    ) -> Result<SafetyWord, DeviceError<SPI::Error>> {
        let command = Command::write_command(address);
        let command_with_word_data = CommandWithWordData::from_parts(command, data);
        let raw_command = command_with_word_data.value().to_be_bytes();
        let mut buffer: [u8; 2] = [0u8; 2];

        self.spi
            .transaction(&mut [
                Operation::Write(&raw_command),
                Operation::DelayNs(RW_DELAY),
                Operation::Read(&mut buffer),
            ])
            .map_err(DeviceError::SpiError)?;

        let raw_safety_word = u16::from_be_bytes(buffer);
        let safety_word = inspect_safety(&raw_command, raw_safety_word);
        logi!("write result{}", raw_safety_word);

        Ok(safety_word)
    }

    fn read_then_mutate<T>(
        &mut self,
        mutator_fn: impl FnOnce(&mut T),
    ) -> Result<SafetyWord, DeviceError<SPI::Error>>
    where
        T: RegisterFromRaw,
    {
        let raw_val = self.read(T::ADDRESS.clone())?;
        let (mut reg_val, _) = resulted_register::<T>(raw_val);

        mutator_fn(&mut reg_val);

        self.write(T::ADDRESS, reg_val.into_u16())
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use crate::{
        crc_table::calc_crc,
        reg::status_reg::{StatusRegisterStructure, StatusReset},
    };
    use embedded_hal::spi::{ErrorKind, ErrorType, SpiDevice};
    use std::vec::Vec;

    #[derive(Debug, PartialEq, Eq)]
    struct MockError;

    impl embedded_hal::spi::Error for MockError {
        fn kind(&self) -> ErrorKind {
            ErrorKind::Other
        }
    }

    struct ExpectedTransaction {
        write: Vec<u8>,
        response: Vec<u8>,
    }

    struct MockSpi {
        expected: Vec<ExpectedTransaction>,
        transaction_count: usize,
    }

    impl MockSpi {
        fn new(expected: Vec<ExpectedTransaction>) -> Self {
            Self {
                expected,
                transaction_count: 0,
            }
        }
    }

    impl ErrorType for MockSpi {
        type Error = MockError;
    }

    impl SpiDevice for MockSpi {
        fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
            let expected = &self.expected[self.transaction_count];

            match operations {
                [
                    Operation::Write(write),
                    Operation::DelayNs(delay),
                    Operation::Read(read),
                ] => {
                    assert_eq!(*write, expected.write.as_slice());
                    assert_eq!(*delay, RW_DELAY);
                    assert_eq!(read.len(), expected.response.len());
                    read.copy_from_slice(&expected.response);
                }
                _ => panic!("expected one write-delay-read transaction"),
            }

            self.transaction_count += 1;
            Ok(())
        }
    }

    fn response(data: u16, command_and_data: [u8; 4]) -> Vec<u8> {
        let crc = calc_crc(&command_and_data);
        let mut response = Vec::from(data.to_be_bytes());
        response.extend_from_slice(&[0x70, crc]);
        response
    }

    #[test]
    fn read_uses_one_transaction_and_validates_crc() {
        let command = [0x80, 0x01];
        let data = 0x1234;
        let expected = ExpectedTransaction {
            write: Vec::from(command),
            response: response(data, [command[0], command[1], 0x12, 0x34]),
        };
        let mut device = DeviceInner::new(MockSpi::new(Vec::from([expected])));

        let raw = device.read(PossibleAddress::StatusRegister).unwrap();

        assert_eq!(raw >> 16, data as u32);
        assert_eq!(device.spi.transaction_count, 1);
    }

    #[test]
    fn write_sends_command_before_data_in_one_transaction() {
        let command_and_data = [0x50, 0x81, 0x08, 0x04];
        let crc = calc_crc(&command_and_data);
        let expected = ExpectedTransaction {
            write: Vec::from(command_and_data),
            response: Vec::from([0x70, crc]),
        };
        let mut device = DeviceInner::new(MockSpi::new(Vec::from([expected])));

        device
            .write(PossibleAddress::Mode2Register, 0x0804)
            .unwrap();

        assert_eq!(device.spi.transaction_count, 1);
    }

    #[test]
    fn read_then_mutate_uses_the_register_associated_address() {
        let read_command = [0x80, 0x01];
        let write_command_and_data = [0x00, 0x01, 0x00, 0x01];
        let read = ExpectedTransaction {
            write: Vec::from(read_command),
            response: response(0, [read_command[0], read_command[1], 0, 0]),
        };
        let write = ExpectedTransaction {
            write: Vec::from(write_command_and_data),
            response: Vec::from([0x70, calc_crc(&write_command_and_data)]),
        };
        let mut device = DeviceInner::new(MockSpi::new(Vec::from([read, write])));

        device
            .read_then_mutate::<StatusRegisterStructure>(|status| {
                status.set_status_reset(StatusReset::ResetHappen);
            })
            .unwrap();

        assert_eq!(device.spi.transaction_count, 2);
    }
}
