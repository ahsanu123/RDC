//! I2C register framing: address-pointer write/read and register/data write.

use embedded_hal::i2c::I2c;

use crate::{error::Cst816sError, register::Address};

pub(crate) const I2C_ADDRESS: u8 = 0x15;

/// Sequential touch-window read, corroborated by InfiniTime's CST816S driver.
/// This framing does not imply hardware sample latching (README: Touch consistency).
pub(crate) fn read_touch_window<I: I2c, const N: usize>(
    i2c: &mut I,
    device_address: u8,
    address: Address,
) -> Result<[u8; N], Cst816sError<I::Error>> {
    let mut bytes = [0; N];
    i2c.write_read(device_address, &[address as u8], &mut bytes)
        .map_err(Cst816sError::Interface)?;
    Ok(bytes)
}

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
    let range = match address {
        Address::NormalScanPeriod => 1..=30,
        Address::InterruptPulseWidth => 1..=200,
        Address::LowPowerScanWindow => 0..=3,
        Address::LowPowerAutoWakeTime => 1..=5,
        Address::LowPowerScanThreshold
        | Address::LowPowerScanFrequency
        | Address::LowPowerScanCurrent => 1..=255,
        _ => 0..=255,
    };
    if !range.contains(&value) {
        return Err(Cst816sError::InvalidRegisterValue { address, value });
    }
    i2c.write(device_address, &[address as u8, value])
        .map_err(Cst816sError::Interface)
}
