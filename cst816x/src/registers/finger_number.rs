use bilge::prelude::*;

/// FingerNum codes (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(TryFromBits, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FingerCount {
    NoFinger = 0,
    OneFinger = 1,
}

/// FingerNum, 0x02 (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(TryFromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct FingerNumber {
    pub count: FingerCount,
}
