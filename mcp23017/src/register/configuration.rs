use bilge::prelude::*;

/// IOCON: global configuration, mirrored at the port A and B addresses.
/// Datasheet: DS20001952D, section 3.5.6, Configuration Register.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Configuration {
    pub unimplemented_bit: bool,
    pub interrupt_active_high: bool,
    pub interrupt_open_drain: bool,
    pub hardware_address_enable: bool,
    pub slew_rate_disabled: bool,
    pub sequential_operation_disabled: bool,
    pub mirror_interrupts: bool,
    pub separate_register_banks: bool,
}

super::register!(Configuration, 0x0a, 0x00);
