use crate::register::{
    AverageCurrent, AveragePower, InternalTemperature, MaximumLoadCurrent, StandbyCurrent,
    Temperature,
};

/// Pure register-result processing; SLUUAC9A sections 4.2 and 4.9--4.14.
#[derive(Default)]
pub(crate) struct Measurements;

impl Measurements {
    pub(crate) fn temperature_centicelsius(&self, value: Temperature) -> i32 {
        i32::from(value.decikelvin()) * 10 - 27315
    }

    pub(crate) fn internal_temperature_centicelsius(&self, value: InternalTemperature) -> i32 {
        i32::from(value.decikelvin()) * 10 - 27315
    }

    pub(crate) fn average_current_milliamps(&self, value: AverageCurrent) -> i16 {
        i16::from_le_bytes(value.raw().to_le_bytes())
    }

    pub(crate) fn standby_current_milliamps(&self, value: StandbyCurrent) -> i16 {
        i16::from_le_bytes(value.raw().to_le_bytes())
    }

    pub(crate) fn maximum_load_current_milliamps(&self, value: MaximumLoadCurrent) -> i16 {
        i16::from_le_bytes(value.raw().to_le_bytes())
    }

    pub(crate) fn average_power_milliwatts(&self, value: AveragePower) -> i16 {
        i16::from_le_bytes(value.raw().to_le_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_conversion_fractional_negative_and_boundaries() {
        for (raw, expected) in [(0, -27315), (2731, -5), (2982, 2505), (65535, 628035)] {
            assert_eq!(Measurements.temperature_centicelsius(raw.into()), expected);
            assert_eq!(
                Measurements.internal_temperature_centicelsius(raw.into()),
                expected
            );
        }
    }

    #[test]
    fn test_signed_measurement_conversion_charge_discharge_and_boundaries() {
        for (raw, expected) in [
            (0, 0),
            (125, 125),
            (0xff38, -200),
            (0x7fff, 32767),
            (0x8000, -32768),
        ] {
            assert_eq!(Measurements.average_current_milliamps(raw.into()), expected);
            assert_eq!(Measurements.standby_current_milliamps(raw.into()), expected);
            assert_eq!(
                Measurements.maximum_load_current_milliamps(raw.into()),
                expected
            );
            assert_eq!(Measurements.average_power_milliwatts(raw.into()), expected);
        }
    }
}
