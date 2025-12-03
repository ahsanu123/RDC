use crate::max31865_error::Max31865Error;

pub trait ReadTrait {
    fn read_u16(&mut self, address: u8) -> u16;
    fn read_u8(&mut self, address: u8) -> u8;
    fn read_n<const N: usize>(&mut self, address: u8, buffer: &mut [u8; N]);
}

pub trait WriteTrait {
    fn write_u8(&mut self, address: u8, data: u8) -> Result<(), Max31865Error>;
}
