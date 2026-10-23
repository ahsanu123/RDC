use crate::register::{
    CellVoltage, ChargeRate, Configuration, HibernateThreshold, ResetVoltage, StateOfCharge,
    VoltageAlert,
};

/// Datasheet Rev. 7, pages 10-14: register-to-physical-value conversions.
#[derive(Default)]
pub(crate) struct Functions;

impl Functions {
    pub(crate) fn voltage(&self, value: CellVoltage) -> f32 {
        f32::from(value.count()) * 0.000_078_125
    }

    pub(crate) fn state_of_charge(&self, value: StateOfCharge) -> f32 {
        f32::from(u16::from(value)) / 256.0
    }

    pub(crate) fn charge_rate(&self, value: ChargeRate) -> f32 {
        f32::from(value.count() as i16) * 0.208
    }

    pub(crate) fn hibernate_threshold(&self, value: HibernateThreshold) -> f32 {
        f32::from(value.hibernate_threshold()) * 0.208
    }

    pub(crate) fn activity_threshold(&self, value: HibernateThreshold) -> f32 {
        f32::from(value.activity_threshold()) * 0.00125
    }

    pub(crate) fn empty_alert_threshold(&self, value: Configuration) -> u8 {
        32 - value.empty_alert_threshold().value()
    }

    pub(crate) fn voltage_alert(&self, value: VoltageAlert) -> (f32, f32) {
        (
            f32::from(value.minimum()) * 0.020,
            f32::from(value.maximum()) * 0.020,
        )
    }

    pub(crate) fn reset_voltage(&self, value: ResetVoltage) -> f32 {
        f32::from(value.threshold().value()) * 0.040
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.001, "{actual} != {expected}");
    }

    #[test]
    fn converts_cell_voltage() {
        for (raw, expected) in [(0, 0.0), (48_000, 3.75), (64_000, 5.0), (65535, 5.119922)] {
            close(Functions.voltage(raw.into()), expected);
        }
    }

    #[test]
    fn converts_state_of_charge_with_fraction() {
        for (raw, expected) in [
            (0, 0.0),
            (0x3280, 50.5),
            (0x6400, 100.0),
            (0xffff, 255.9961),
        ] {
            close(Functions.state_of_charge(raw.into()), expected);
        }
    }

    #[test]
    fn converts_signed_charge_rate() {
        for (raw, expected) in [
            (0, 0.0),
            (100, 20.8),
            (0xff9c, -20.8),
            (0x8000, -6815.744),
            (0x7fff, 6815.536),
        ] {
            close(Functions.charge_rate(raw.into()), expected);
        }
    }

    #[test]
    fn converts_hibernate_threshold() {
        close(Functions.hibernate_threshold(0x8030.into()), 26.624);
        close(Functions.hibernate_threshold(0xffff.into()), 53.04);
        close(Functions.hibernate_threshold(0.into()), 0.0);
    }

    #[test]
    fn converts_activity_threshold() {
        close(Functions.activity_threshold(0x8030.into()), 0.06);
        close(Functions.activity_threshold(0xffff.into()), 0.31875);
        close(Functions.activity_threshold(0.into()), 0.0);
    }

    #[test]
    fn converts_empty_alert_threshold() {
        for (raw, percent) in [(0x971c, 4), (0, 32), (31, 1)] {
            assert_eq!(Functions.empty_alert_threshold(raw.into()), percent);
        }
    }

    #[test]
    fn converts_voltage_alert() {
        let (minimum, maximum) = Functions.voltage_alert(0x96d2.into());
        close(minimum, 3.0);
        close(maximum, 4.2);
        let (minimum, maximum) = Functions.voltage_alert(0x00ff.into());
        close(minimum, 0.0);
        close(maximum, 5.1);
    }

    #[test]
    fn converts_reset_voltage() {
        close(Functions.reset_voltage(0x96a5.into()), 3.0);
        close(Functions.reset_voltage(0xff00.into()), 5.08);
        close(Functions.reset_voltage(0x01ff.into()), 0.0);
    }
}
