use bilge::prelude::*;

/// DataClass() selector; SLUUAC9A section 5.3.
#[bitsize(8)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct DataClass {
    pub identifier: u8,
}

/// Documented data-memory subclasses; SLUUAC9A tables 6-2 through 6-6.
#[bitsize(8)]
#[derive(Clone, Copy, Debug, TryFromBits, PartialEq, Eq)]
pub enum DataSubclass {
    Safety = 2,
    ChargeTermination = 36,
    Data = 48,
    Discharge = 49,
    Registers = 64,
    Power = 68,
    ImpedanceTrackConfiguration = 80,
    CurrentThresholds = 81,
    State = 82,
    ResistanceTable = 89,
    CalibrationData = 104,
    CoulombCounterCalibration = 105,
    Current = 107,
    SecurityCodes = 112,
}
