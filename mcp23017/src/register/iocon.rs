use bilge::prelude::*;

/// IOCON: global configuration, mirrored at the port A and B addresses.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Iocon {
    pub reserved: bool,
    pub interrupt_active_high: bool,
    pub interrupt_open_drain: bool,
    pub hardware_address_enable: bool,
    pub slew_rate_disabled: bool,
    pub sequential_operation_disabled: bool,
    pub mirror_interrupts: bool,
    pub bank: bool,
}

super::register!(Iocon, 0x0a, 0x00);
