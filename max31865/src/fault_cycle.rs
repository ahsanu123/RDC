use bilge::prelude::*;

/// Configuration D3:D2, MAX31865 Rev 3, Table 3, page 14.
#[bitsize(2)]
#[derive(FromBits, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FaultCycle {
    #[default]
    None = 0,
    Auto,
    ManualRun,
    ManualFinish,
}
