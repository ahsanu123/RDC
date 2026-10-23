use crate::{Register, error::Max31865Err};

pub trait ReadTrait {
    type InterfaceError;

    fn read_u16(&mut self, address: Register) -> Result<u16, Max31865Err<Self::InterfaceError>>;
    fn read_u8(&mut self, address: Register) -> Result<u8, Max31865Err<Self::InterfaceError>>;
    fn read_n<const N: usize>(
        &mut self,
        address: Register,
        buffer: &mut [u8; N],
    ) -> Result<(), Max31865Err<Self::InterfaceError>>;
}

pub trait WriteTrait {
    type InterfaceError;

    fn write_u8(
        &mut self,
        address: Register,
        data: u8,
    ) -> Result<(), Max31865Err<Self::InterfaceError>>;
}
