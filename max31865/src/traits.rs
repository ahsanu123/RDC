use crate::{Register, max31865_error::Max31865Error};

pub trait ReadTrait {
    fn read_u16(&mut self, address: Register) -> u16;
    fn read_u8(&mut self, address: Register) -> u8;
    fn read_n<const N: usize>(&mut self, address: Register, buffer: &mut [u8; N]);
}

pub trait WriteTrait {
    fn write_u8(&mut self, address: Register, data: u8) -> Result<(), Max31865Error>;
}
