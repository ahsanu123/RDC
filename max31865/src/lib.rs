#![no_std]

mod fault_cycle;
mod max31865_error;
mod numwires;
mod regiter;
mod traits;

use embedded_hal::{
    delay::DelayNs,
    spi::{Operation, SpiDevice},
};

use crate::{
    fault_cycle::FaultCycle,
    max31865_error::Max31865Error,
    numwires::Numwires,
    regiter::Register,
    traits::{ReadTrait, WriteTrait},
};

// Max31865 is generic for SpiDevice and DelayNs trait
pub struct Max31865<SPI, Delay>
where
    SPI: SpiDevice,
    Delay: DelayNs,
{
    spi: SPI,
    delay: Delay,
    ref_resistor: f32,
    rtd_nominal: f32,
}

impl<SPI, Delay> WriteTrait for Max31865<SPI, Delay>
where
    SPI: SpiDevice,
    Delay: DelayNs,
{
    fn write_u8(&mut self, address: u8, data: u8) -> Result<(), Max31865Error> {
        let data = data | 0x80;
        self.spi
            .write(&[address, data])
            .map_err(|_e| Max31865Error::WriteError)
    }
}

impl<SPI, Delay> ReadTrait for Max31865<SPI, Delay>
where
    SPI: SpiDevice,
    Delay: DelayNs,
{
    fn read_u16(&mut self, address: u8) -> u16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.read_n::<2>(address, &mut buffer);

        u16::from_be_bytes(buffer)
    }

    fn read_u8(&mut self, address: u8) -> u8 {
        let mut buffer: [u8; 1] = [0; 1];
        self.read_n::<1>(address, &mut buffer);

        buffer[0]
    }

    fn read_n<const N: usize>(&mut self, address: u8, buffer: &mut [u8; N]) {
        let address = address & 0x7F;

        let _ = self
            .spi
            .transaction(&mut [Operation::Write(&[address]), Operation::Read(buffer)])
            .map_err(|_e| Max31865Error::WriteError);
    }
}

impl<SPI, Delay> Max31865<SPI, Delay>
where
    SPI: SpiDevice,
    Delay: DelayNs,
{
    pub fn new(spi: SPI, delay: Delay, wire: u8, rtd_nominal: f32, ref_resistor: f32) -> Self {
        let mut instance = Self {
            spi,
            delay,
            rtd_nominal,
            ref_resistor,
        };

        instance.set_wires(wire);
        instance.enable_bias(false);
        instance.auto_convert(false);
        instance.set_threshold(0, 0xFFFF);
        instance.clear_fault();

        instance
    }

    pub fn clear_fault(&mut self) {
        let mut value = self.read_u8(Register::MAX31865_CONFIG_REG);

        value &= !0x2C;
        value |= Register::MAX31865_CONFIG_FAULTSTAT;

        let _ = self.write_u8(Register::MAX31865_CONFIG_REG, value);
    }

    pub fn enable_bias(&mut self, enable: bool) {
        let mut value = self.read_u8(Register::MAX31865_CONFIG_REG);
        if enable {
            value |= Register::MAX31865_CONFIG_BIAS;
        } else {
            value |= !Register::MAX31865_CONFIG_BIAS;
        }

        let _ = self.write_u8(Register::MAX31865_CONFIG_REG, value);
    }

    pub fn read_fault(&mut self, cycle: FaultCycle) -> u8 {
        if cycle != FaultCycle::None {
            let mut config_reg = self.read_u8(Register::MAX31865_CONFIG_REG);
            config_reg &= 0x11;

            match cycle {
                FaultCycle::Auto => {
                    let _ = self.write_u8(Register::MAX31865_CONFIG_REG, config_reg | 0b10000100);
                    self.delay.delay_ms(1);
                }
                FaultCycle::ManualRun => {
                    let _ = self.write_u8(Register::MAX31865_CONFIG_REG, config_reg | 0b10001000);
                }
                FaultCycle::ManualFinish => {
                    let _ = self.write_u8(Register::MAX31865_CONFIG_REG, config_reg | 0b10001100);
                }

                _ => {}
            }
        }
        self.read_u8(Register::MAX31865_FAULTSTAT_REG)
    }
    pub fn auto_convert(&mut self, enable: bool) {
        let mut value = self.read_u8(Register::MAX31865_CONFIG_REG);
        if enable {
            value |= Register::MAX31865_CONFIG_MODEAUTO;
        } else {
            value &= !Register::MAX31865_CONFIG_MODEAUTO;
        }
        let _ = self.write_u8(Register::MAX31865_CONFIG_REG, value);
    }

    pub fn enable_50hz(&mut self, enable: bool) {
        let mut value = self.read_u8(Register::MAX31865_CONFIG_REG);
        if enable {
            value |= Register::MAX31865_CONFIG_FILT50HZ;
        } else {
            value &= !Register::MAX31865_CONFIG_FILT50HZ;
        }
        let _ = self.write_u8(Register::MAX31865_CONFIG_REG, value);
    }
    pub fn read_rtd(&mut self) -> u16 {
        self.clear_fault();
        self.enable_bias(true);
        self.delay.delay_ms(10);

        let mut value = self.read_u8(Register::MAX31865_CONFIG_REG);
        value |= Register::MAX31865_CONFIG_1SHOT;

        let _ = self.write_u8(Register::MAX31865_CONFIG_REG, value);
        self.delay.delay_ms(65);

        let mut rtd = self.read_u16(Register::MAX31865_RTDMSB_REG);

        self.enable_bias(false);
        rtd >>= 1;

        rtd
    }

    pub fn set_threshold(&mut self, lower: u16, upper: u16) {
        let _ = self.write_u8(Register::MAX31865_LFAULTLSB_REG, (lower & 0xFF) as u8);
        let _ = self.write_u8(Register::MAX31865_LFAULTMSB_REG, (lower >> 8) as u8);

        let _ = self.write_u8(Register::MAX31865_HFAULTLSB_REG, (upper & 0xFF) as u8);
        let _ = self.write_u8(Register::MAX31865_HFAULTMSB_REG, (upper >> 8) as u8);
    }

    pub fn get_lower_threshold(&mut self) -> u16 {
        self.read_u16(Register::MAX31865_LFAULTMSB_REG)
    }

    pub fn get_upper_threshold(&mut self) -> u16 {
        self.read_u16(Register::MAX31865_HFAULTMSB_REG)
    }

    pub fn set_wires(&mut self, wire: u8) {
        let mut value = self.read_u8(Register::MAX31865_CONFIG_REG);
        if wire == Numwires::MAX31865_3_WIRE {
            value |= Register::MAX31865_CONFIG_3WIRE;
        } else {
            value &= !Register::MAX31865_CONFIG_3WIRE;
        }
        let _ = self.write_u8(Register::MAX31865_CONFIG_REG, value);
    }

    pub fn temperature(&mut self) -> f32 {
        let rtd_raw: f32 = self.read_rtd() as f32;

        let mut rt = rtd_raw / 32768.0;
        rt *= self.ref_resistor;

        let z1 = -Register::RTD_A;
        let z2 = Register::RTD_A * Register::RTD_A - (4.0 * Register::RTD_B);
        let z3 = (4.0 * Register::RTD_B) / self.rtd_nominal;
        let z4 = 2.0 * Register::RTD_B;

        let mut temp = z2 + (z3 * rt);
        temp = (libm::sqrtf(temp) + z1) / z4;

        if temp >= 0.0 {
            return temp;
        }

        rt /= self.rtd_nominal;
        rt *= 100.0;

        let mut rpoly = rt;

        temp = -242.02;
        temp += 2.2228 * rpoly;
        rpoly *= rt; // square
        temp += 2.5859e-3 * rpoly;
        rpoly *= rt; // ^3
        temp -= 4.8260e-6 * rpoly;
        rpoly *= rt; // ^4
        temp -= 2.8183e-8 * rpoly;
        rpoly *= rt; // ^5
        temp += 1.5243e-10 * rpoly;

        temp
    }

    pub fn get_resistance(&mut self) -> f32 {
        let rtd = self.read_rtd() as f32;
        let ratio = rtd * 32768.0;

        self.ref_resistor * ratio
    }

    pub fn get_ratio(&mut self) -> f32 {
        let rtd = self.read_rtd() as f32;
        rtd * 32768.0
    }
}
