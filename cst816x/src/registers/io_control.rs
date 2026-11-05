use bilge::prelude::*;

/// En1v8 voltage selection (CST816S register declaration, page 5).
#[bitsize(1)]
#[derive(FromBits, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IoVoltage {
    Vdd = 0,
    Volts1_8 = 1,
}

/// IIC_OD pin driver selection (CST816S register declaration, page 5).
#[bitsize(1)]
#[derive(FromBits, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TwoWireDriverMode {
    PullUpResistor = 0,
    OpenDrain = 1,
}

/// IOCtl, 0xFD (CST816S register declaration, page 5).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct IoControl {
    pub voltage: IoVoltage,
    pub two_wire_driver_mode: TwoWireDriverMode,
    pub irq_soft_reset_enabled: bool,
    pub unspecified: u5,
}
