use bitflags::bitflags;

bitflags! {
    pub struct Register: u8 {
        const CONFIG_REG= 0x00;

        const CONFIG_FAULT_DETECTION_CYCLE_CONTROL= 0x0C;
        const CONFIG_BIAS= 0x80;
        const CONFIG_MODEAUTO = 0x40;
        const CONFIG_MODEOFF= 0x00;
        const CONFIG_1SHOT= 0x20;
        const CONFIG_3WIRE= 0x10;
        const CONFIG_24WIRE= 0x00;
        const CONFIG_FAULTSTAT= 0x02;
        const CONFIG_FILT50HZ= 0x01;
        const CONFIG_FILT60HZ= 0x00;

        const RTDMSB_REG= 0x01;
        const RTDLSB_REG= 0x02;
        const HFAULTMSB_REG= 0x03;
        const HFAULTLSB_REG= 0x04;
        const LFAULTMSB_REG= 0x05;
        const LFAULTLSB_REG= 0x06;
        const FAULTSTAT_REG= 0x07;

        const FAULT_HIGHTHRESH= 0x80;
        const FAULT_LOWTHRESH= 0x40;
        const FAULT_REFINLOW= 0x20;
        const FAULT_REFINHIGH= 0x10;
        const FAULT_RTDINLOW= 0x08;
        const FAULT_OVUV= 0x04;
    }
}

impl From<Register> for u8 {
    fn from(value: Register) -> Self {
        value.bits()
    }
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        Register::from_bits_retain(value)
    }
}

impl Register {
    pub const RTD_A: f32 = 3.9083e-3;
    pub const RTD_B: f32 = -5.775e-7;
}
