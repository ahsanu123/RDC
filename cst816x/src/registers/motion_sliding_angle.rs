use bilge::prelude::*;

/// MotionSlAngle, 0xEF (CST816S register declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct MotionSlidingAngle {
    /// Raw tan(c) * 10 setting; the PDF does not specify angle units or rounding.
    pub tangent_times_ten: u8,
}
