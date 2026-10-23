use embedded_hal::{delay::DelayNs, i2c::I2c};

pub(crate) const I2C_ADDRESS: u8 = 0x55;
const POWER_UP_DELAY_MS: u32 = 250;
const BUS_FREE_TIME_US: u32 = 66;
const STANDARD_READ_INTERVAL_MS: u32 = 500;
const READ_WRITE_UPDATE_MS: u32 = 2000;

/// I2C framing/timing from SLUSBH1C section 9.5.4, including fast-mode writes.
pub(crate) struct I2cInterface<I2C, DELAY> {
    i2c: I2C,
    delay: DELAY,
}

impl<I: I2c, D: DelayNs> I2cInterface<I, D> {
    pub(crate) fn new(i2c: I, mut delay: D) -> Self {
        delay.delay_ms(POWER_UP_DELAY_MS);
        Self { i2c, delay }
    }

    pub(crate) fn release(self) -> (I, D) {
        (self.i2c, self.delay)
    }

    pub(crate) fn delay_ms(&mut self, milliseconds: u32) {
        self.delay.delay_ms(milliseconds);
    }

    pub(crate) fn read(&mut self, address: u8, bytes: &mut [u8]) -> Result<(), I::Error> {
        let result = self.i2c.write_read(I2C_ADDRESS, &[address], bytes);
        self.delay.delay_us(BUS_FREE_TIME_US);
        result
    }

    pub(crate) fn read_word(&mut self, address: u8) -> Result<u16, I::Error> {
        let mut bytes = [0; 2];
        let result = self.read(address, &mut bytes);
        if address <= 0x30 {
            self.delay.delay_ms(STANDARD_READ_INTERVAL_MS);
        }
        result.map(|()| u16::from_le_bytes(bytes))
    }

    pub(crate) fn write_byte(&mut self, address: u8, byte: u8) -> Result<(), I::Error> {
        let result = self.i2c.write(I2C_ADDRESS, &[address, byte]);
        self.delay.delay_us(BUS_FREE_TIME_US);
        result
    }

    pub(crate) fn write_word(&mut self, address: u8, value: u16) -> Result<(), I::Error> {
        let [low, high] = value.to_le_bytes();
        let result = self
            .write_byte(address, low)
            .and_then(|()| self.write_byte(address + 1, high));
        self.delay.delay_ms(READ_WRITE_UPDATE_MS);
        result
    }
}
