use bilge::prelude::*;

/// Continuous measurement frequency (Rev A, pages 15-16).
#[bitsize(3)]
#[derive(FromBits, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ContinuousFrequency {
    #[default]
    Off = 0,
    Hertz1 = 1,
    Hertz10 = 2,
    Hertz20 = 3,
    Hertz50 = 4,
    Hertz100 = 5,
    Hertz200 = 6,
    Hertz1000 = 7,
}

/// Number of measurements between periodic SET pulses (Rev A, page 16).
#[bitsize(3)]
#[derive(FromBits, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PeriodicSetInterval {
    #[default]
    Every1 = 0,
    Every25 = 1,
    Every75 = 2,
    Every100 = 3,
    Every250 = 4,
    Every500 = 5,
    Every1000 = 6,
    Every2000 = 7,
}

/// Internal control 2, continuous acquisition and periodic SET (Rev A, page 16).
#[bitsize(8)]
#[derive(FromBits, DebugBits, Default, Clone, Copy, PartialEq, Eq)]
pub struct Control2 {
    pub frequency: ContinuousFrequency,
    pub continuous_measurement_enabled: bool,
    pub periodic_set_interval: PeriodicSetInterval,
    pub periodic_set_enabled: bool,
}
