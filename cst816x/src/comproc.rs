//! I2C register framing: address-pointer write/read and register/data write.

use embedded_hal::i2c::I2c;

use crate::{error::Cst816sError, registers::Address};

pub(crate) const I2C_ADDRESS: u8 = 0x15;

pub(crate) fn read<I: I2c>(
    i2c: &mut I,
    device_address: u8,
    address: Address,
) -> Result<u8, Cst816sError<I::Error>> {
    let mut bytes = [0];
    i2c.write_read(device_address, &[address as u8], &mut bytes)
        .map_err(Cst816sError::Interface)?;
    Ok(bytes[0])
}

pub(crate) fn write<I: I2c>(
    i2c: &mut I,
    device_address: u8,
    address: Address,
    value: u8,
) -> Result<(), Cst816sError<I::Error>> {
    i2c.write(device_address, &[address as u8, value])
        .map_err(Cst816sError::Interface)
}
