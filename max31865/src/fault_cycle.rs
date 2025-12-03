#[derive(PartialEq, Eq)]
pub enum FaultCycle {
    None = 0,
    Auto,
    ManualRun,
    ManualFinish,
}
