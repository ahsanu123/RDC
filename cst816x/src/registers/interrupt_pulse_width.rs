use bilge::prelude::*;

/// IrqPluseWidth, 0xED (CST816S register declaration, page 3).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Clone, Copy, PartialEq, Eq)]
pub struct InterruptPulseWidth {
    /// Raw 0.1 ms units; documented range: 1-200.
    pub tenths_of_millisecond: u8,
}

impl Default for InterruptPulseWidth {
    fn default() -> Self {
        Self::new(10)
    }
}
