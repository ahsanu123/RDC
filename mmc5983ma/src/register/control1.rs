use bilge::prelude::*;

/// Decimation bandwidth and nominal measurement time (Rev A, page 15).
#[bitsize(2)]
#[derive(FromBits, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Bandwidth {
    #[default]
    Hertz100 = 0,
    Hertz200 = 1,
    Hertz400 = 2,
    Hertz800 = 3,
}

/// Internal control 1 (Rev A, page 15). Y/Z inhibit requires both bits high.
#[bitsize(8)]
#[derive(FromBits, DebugBits, Default, Clone, Copy, PartialEq, Eq)]
pub struct Control1 {
    pub bandwidth: Bandwidth,
    pub x_inhibited: bool,
    pub yz_inhibited: u2,
    pub reserved: u2,
    pub software_reset: bool,
}
