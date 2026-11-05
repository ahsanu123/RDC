use bilge::prelude::*;

/// IrqCtl, 0xFA (CST816S register declaration, pages 4-5).
///
/// Uses the field descriptions: EnMotion is bit 4 and OnceWLP is bit 0.
/// The page 4 drawing conflicts with these assignments; see the README.
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct InterruptControl {
    pub single_pulse_on_long_press: bool,
    pub unspecified: u3,
    pub gesture_interrupt_enabled: bool,
    pub touch_change_interrupt_enabled: bool,
    pub touch_interrupt_enabled: bool,
    pub interrupt_test_enabled: bool,
}
