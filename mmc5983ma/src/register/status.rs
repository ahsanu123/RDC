use bilge::prelude::*;

/// Status register, including write-one-to-clear measurement interrupts
/// (Rev A, page 14). Reading does not acknowledge interrupts.
#[bitsize(8)]
#[derive(FromBits, DebugBits, Default, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    pub magnetic_measurement_done: bool,
    pub temperature_measurement_done: bool,
    pub reserved_low: u2,
    pub otp_read_done: bool,
    pub reserved_high: u3,
}
