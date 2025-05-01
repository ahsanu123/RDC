pub struct Register;
impl Register {
    pub const VCELL: u8 = 0x02;
    pub const SOC: u8 = 0x04;
    pub const MODE: u8 = 0x06;
    pub const VERSION: u8 = 0x08;
    pub const HIBRT: u8 = 0x0A;
    pub const CONFIG: u8 = 0x0C;
    pub const VALRT: u8 = 0x14;
    pub const CRATE: u8 = 0x16;
    pub const VRESET: u8 = 0x18;
    pub const STATUS: u8 = 0x1A;
    pub const COMMAND: u8 = 0xFE;
}

pub struct Command;
impl Command {
    pub const POR_43_44: u16 = 0x0054;
    pub const POR_X8_X9: u16 = 0x5400;
    pub const QUICK_START: u16 = 0x4000;
}
