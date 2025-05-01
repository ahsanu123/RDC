use crate::{
    register_access::{Command, Register},
    types::Error,
};
use embedded_hal::i2c::I2c;

pub const ADDR: u8 = 0b011_0110;

#[derive(Debug)]
pub struct MAX17048<I2C> {
    i2c: I2C,
}

impl<I2C, E> MAX17048<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        MAX17048 { i2c }
    }

    pub fn destroy(self) -> I2C {
        self.i2c
    }

    pub fn quick_start(&mut self) -> Result<(), Error<E>> {
        self.write_register(Register::MODE, Command::QUICK_START)
    }

    pub fn version(&mut self) -> Result<u16, Error<E>> {
        self.read_register(Register::VERSION)
    }

    /**
    *   The ICs have a low-power hibernate mode that can accurately
        fuel gauge the battery when the charge/discharge
        rate is low. By default, the device automatically enters
        and exits the hibernate mode according to the charge/
        discharge rate, which minimizes quiescent current (below
        5µA) without compromising fuel-gauge accuracy. The ICs
        can be forced into hibernate or active modes. Force the
        IC into hibernate mode to reduce power consumption in
        applications with less than C/4-rate maximum loading.
        For applications with higher loading, Maxim recommends
        the default configuration of automatic control of hibernate mode.

        In hibernate mode, the device reduces its ADC conversion
        period and SOC update to once per 45s. See the
        HIBRT Register (0x0A) section for details on how the IC
        automatically enters and exits hibernate mode.
    * */
    pub fn set_hibernate_threshold(&mut self) -> Result<(), Error<E>> {
        todo!()
    }
    pub fn get_hibernate_threshold(&mut self) -> Result<(), Error<E>> {
        todo!()
    }

    pub fn set_config(&mut self) -> Result<(), Error<E>> {
        todo!()
    }
    pub fn get_config(&mut self) -> Result<(), Error<E>> {
        todo!()
    }

    pub fn set_status(&mut self) -> Result<(), Error<E>> {
        todo!()
    }
    pub fn get_status(&mut self) -> Result<(), Error<E>> {
        todo!()
    }

    pub fn soc(&mut self) -> Result<f32, Error<E>> {
        let state_of_charge = self.read_register(Register::SOC)?;

        Ok(f32::from(state_of_charge))
    }

    pub fn voltage(&mut self) -> Result<f32, Error<E>> {
        let voltage = self.read_register(Register::VCELL)?;

        // max applicable voltage for MAX17048 vcell  is 5V
        // its resolution is 78.125uV, so for full scale
        // its only need 64000 decimal
        Ok(f32::from(voltage) * 5.0 / 64000.0)
    }

    pub fn charge_rate(&mut self) -> Result<f32, Error<E>> {
        let rate = self.read_register(Register::CRATE)? as i16;

        Ok(f32::from(rate) * 0.208)
    }
}

impl<I2C, E> MAX17048<I2C>
where
    I2C: I2c<Error = E>,
{
    // Register Summary (datasheet page 10)
    //
    // All registers must be written and read as 16-bit words;

    pub(crate) fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<E>> {
        let payload: [u8; 3] = [register, ((data & 0xFF00) >> 8) as u8, (data & 0xFF) as u8];

        self.i2c.write(ADDR, &payload).map_err(Error::I2C)
    }

    pub(crate) fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0; 2];

        self.i2c
            .write_read(ADDR, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok((u16::from(data[0]) << 8) | u16::from(data[1])))
    }
}
