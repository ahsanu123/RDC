use bilge::prelude::*;

/// SG_RESULT, load estimate; larger means more torque headroom. Rev. 1.09, section 5.3.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct StallDetectionResult {
    pub result: u10,
    pub unused: u22,
}

super::register!(StallDetectionResult, 0x41, None);
impl super::ReadableRegister for StallDetectionResult {}
