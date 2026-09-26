use crate::{
    TLE5012B,
    communication::{
        command::{Command, CommandWithWordData},
        possible_address::PossibleAddress,
    },
    crc_table::does_crc_match,
};
use embedded_hal::spi::Operation;

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

pub trait DeviceTrait<SPI, DELAY>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
{
    fn read(&mut self, address: PossibleAddress) -> Result<u32, SPI::Error>;
    fn write(&mut self, address: PossibleAddress, data: u16) -> Result<u16, SPI::Error>;
}

impl<SPI, DELAY> DeviceTrait<SPI, DELAY> for TLE5012B<SPI, DELAY>
where
    SPI: embedded_hal::spi::SpiDevice,
    DELAY: embedded_hal::delay::DelayNs,
{
    fn read(&mut self, address: PossibleAddress) -> Result<u32, SPI::Error> {
        let command = Command::read_command(address);
        let raw_command: [u8; 2] = command.value().to_be_bytes();

        // TODO:
        // if something not work, check here!!
        // word (16)
        // for data and safety word
        let mut buffer: [u8; 4] = [0u8; 4];

        self.spi
            .transaction(&mut [Operation::Write(&raw_command)])?;

        // minimum 130 ns
        self.delay.delay_ns(RW_DELAY);
        self.spi.transaction(&mut [Operation::Read(&mut buffer)])?;

        let buffer = u32::from_be_bytes(buffer);

        // TODO: add and test this after real hardware made
        // let crc = does_crc_match::<4>(&raw_command, buffer);
        // if !crc.is_value_match {
        //     loge!(
        //         "crc not match, expected {}, got {}",
        //         crc.expected_value,
        //         crc.real_value
        //     );
        // }

        logi!("read result{}", buffer);
        Ok(buffer)
    }

    fn write(&mut self, address: PossibleAddress, data: u16) -> Result<u16, SPI::Error> {
        let command = Command::write_command(address);
        let command_with_word_data = CommandWithWordData::new(command, data);
        let raw_command: [u8; 4] = command_with_word_data.value().to_be_bytes();

        let mut buffer: [u8; 2] = [0u8, 2];

        self.spi
            .transaction(&mut [Operation::Write(&raw_command)])?;

        self.delay.delay_ns(RW_DELAY);

        // TODO:
        // if something not work, check here!!
        // word (16)
        // for data and safety word
        self.spi.transaction(&mut [Operation::Read(&mut buffer)])?;
        let buffer = u16::from_be_bytes(buffer);

        let crc = does_crc_match::<4>(&raw_command, buffer);
        if !crc.is_value_match {
            // NOTE: just log error for now
            loge!(
                "crc not match, expected {}, got {}",
                crc.expected_value,
                crc.real_value
            );
        }

        logi!("write result{}", buffer);

        Ok(buffer)
    }
}
