use bilge::prelude::*;

/// MotionMask, 0xEC (CST816S register declaration, page 2).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct MotionMask {
    pub double_click_enabled: bool,
    pub continuous_up_down_enabled: bool,
    pub continuous_left_right_enabled: bool,
    pub unspecified: u5,
}
