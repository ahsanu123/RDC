use bilge::prelude::*;

/// Internal control 3, saturation test currents and three-wire SPI
/// (Rev A, page 16). This driver supports four-wire SPI only.
#[bitsize(8)]
#[derive(FromBits, DebugBits, Default, Clone, Copy, PartialEq, Eq)]
pub struct Control3 {
    pub reserved_low: bool,
    pub positive_self_test_enabled: bool,
    pub negative_self_test_enabled: bool,
    pub reserved_middle: u3,
    pub three_wire_spi_enabled: bool,
    pub reserved_high: bool,
}
