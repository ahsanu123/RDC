use bilge::prelude::*;

/// GestureID codes (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(TryFromBits, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gesture {
    NoGesture = 0x00,
    SlideUp = 0x01,
    SlideDown = 0x02,
    SlideLeft = 0x03,
    SlideRight = 0x04,
    SingleClick = 0x05,
    DoubleClick = 0x0B,
    LongPress = 0x0C,
}

/// GestureID, 0x01 (CST816S register declaration, page 1).
#[bitsize(8)]
#[derive(TryFromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct GestureIdentifier {
    pub gesture: Gesture,
}
