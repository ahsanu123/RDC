use embedded_hal::i2c::I2c;

use crate::{ADDR, register::ModelAccess};

pub(crate) fn read_word<I: I2c>(bus: &mut I, address: u8) -> Result<u16, I::Error> {
    let mut bytes = [0; 2];
    bus.write_read(ADDR, &[address], &mut bytes)?;
    let word = u16::from_be_bytes(bytes);
    #[cfg(feature = "defmt")]
    defmt::trace!("MAX17048 read {=u8:#x}: {=u16:#x}", address, word);
    Ok(word)
}

pub(crate) fn write_word<I: I2c>(bus: &mut I, address: u8, word: u16) -> Result<(), I::Error> {
    let [high, low] = word.to_be_bytes();
    #[cfg(feature = "defmt")]
    defmt::trace!("MAX17048 write {=u8:#x}: {=u16:#x}", address, word);
    bus.write(ADDR, &[address, high, low])
}

pub(crate) fn write_byte<I: I2c>(bus: &mut I, address: u8, byte: u8) -> Result<(), I::Error> {
    bus.write(ADDR, &[address, byte])
}

pub(crate) fn write_model_access<I: I2c>(bus: &mut I, key: ModelAccess) -> Result<(), I::Error> {
    write_byte(bus, ModelAccess::LOW_ADDRESS, key.low_key())?;
    write_byte(bus, ModelAccess::HIGH_ADDRESS, key.high_key())
}
