pub struct Register;
impl Register {
    pub const MAX31865_CONFIG_REG: u8 = 0x00;
    pub const MAX31865_CONFIG_BIAS: u8 = 0x80;
    pub const MAX31865_CONFIG_MODEAUTO: u8 = 0x40;
    pub const MAX31865_CONFIG_MODEOFF: u8 = 0x00;
    pub const MAX31865_CONFIG_1SHOT: u8 = 0x20;
    pub const MAX31865_CONFIG_3WIRE: u8 = 0x10;
    pub const MAX31865_CONFIG_24WIRE: u8 = 0x00;
    pub const MAX31865_CONFIG_FAULTSTAT: u8 = 0x02;
    pub const MAX31865_CONFIG_FILT50HZ: u8 = 0x01;
    pub const MAX31865_CONFIG_FILT60HZ: u8 = 0x00;

    pub const MAX31865_RTDMSB_REG: u8 = 0x01;
    pub const MAX31865_RTDLSB_REG: u8 = 0x02;
    pub const MAX31865_HFAULTMSB_REG: u8 = 0x03;
    pub const MAX31865_HFAULTLSB_REG: u8 = 0x04;
    pub const MAX31865_LFAULTMSB_REG: u8 = 0x05;
    pub const MAX31865_LFAULTLSB_REG: u8 = 0x06;
    pub const MAX31865_FAULTSTAT_REG: u8 = 0x07;

    pub const MAX31865_FAULT_HIGHTHRESH: u8 = 0x80;
    pub const MAX31865_FAULT_LOWTHRESH: u8 = 0x40;
    pub const MAX31865_FAULT_REFINLOW: u8 = 0x20;
    pub const MAX31865_FAULT_REFINHIGH: u8 = 0x10;
    pub const MAX31865_FAULT_RTDINLOW: u8 = 0x08;
    pub const MAX31865_FAULT_OVUV: u8 = 0x04;

    pub const RTD_A: f32 = 3.9083e-3;
    pub const RTD_B: f32 = -5.775e-7;
}
