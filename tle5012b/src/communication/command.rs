use bilge::prelude::*;

use crate::communication::possible_address::PossibleAddress;
#[bitsize(16)]
#[derive(TryFromBits)]
// LSB First
pub struct Command {
    // NOTE :
    // number_of_data_words is ic feature to
    // automatic read from start adress
    // which mean if we pass 4 as value
    // to read data at address 0x00
    // it will return it will return 4 data
    // containing data from address 0x00
    // 0x01, 0x02, 0x03
    // addresses is auto incremented by ic
    pub number_of_data_words: u4,

    pub address: PossibleAddress,
    pub update_register: PossibleUpdateOperation,
    pub lock: PossibleLock,
    pub operation: ICOperation,
}

impl Command {
    pub fn read_command(address: PossibleAddress) -> Self {
        let raw_addr = address.clone() as u16;

        let lock = if (0x05..=0x11).contains(&raw_addr) {
            PossibleLock::ConfigurationAccess
        } else {
            PossibleLock::OperationAccess
        };

        Command::new(
            u4::new(1),
            address,
            PossibleUpdateOperation::AccessOnly,
            lock,
            ICOperation::Read,
        )
    }

    pub fn write_command(address: PossibleAddress) -> Self {
        let mut mutated_read_command = Self::read_command(address);

        mutated_read_command.set_operation(ICOperation::Write);

        mutated_read_command
    }

    pub fn value(&self) -> u16 {
        self.value
    }
}

#[bitsize(32, new = pub)]
#[derive(TryFromBits)]
pub struct CommandWithWordData {
    pub word_data: u16,
    pub command: Command,
}

impl CommandWithWordData {
    pub fn from_parts(command: Command, word_data: u16) -> Self {
        Self::new(word_data, command)
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum PossibleUpdateOperation {
    AccessOnly = 0,
    AccessThenUpdate,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ICOperation {
    Write = 0,
    Read,
}

#[bitsize(4)]
#[derive(TryFromBits)]
pub enum PossibleLock {
    OperationAccess = 0b0000,
    ConfigurationAccess = 0b1010,
}
