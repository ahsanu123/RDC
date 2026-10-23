use crate::{
    DebugValue,
    error::HardwareErr,
    register::{FaultStatus, Register, RtdResistance},
};

const ADC_FULL_SCALE: f32 = 32768.0;

/// Ratio/temperature processing from Rev 3 Table 5 and the pre-existing RTD curve.
#[derive(Default)]
pub(crate) struct RtdConversion {
    pub(crate) reference_resistor: f32,
    pub(crate) nominal_resistance: f32,
}

impl RtdConversion {
    pub(crate) fn ratio(&self, register: RtdResistance) -> f32 {
        f32::from(register.count().value()) / ADC_FULL_SCALE
    }

    pub(crate) fn resistance(&self, register: RtdResistance) -> f32 {
        self.ratio(register) * self.reference_resistor
    }

    pub(crate) fn temperature(&self, register: RtdResistance) -> Option<f32> {
        let mut rt = self.resistance(register);
        let z1 = -Register::RTD_A;
        let z2 = Register::RTD_A * Register::RTD_A - (4.0 * Register::RTD_B);
        let z3 = (4.0 * Register::RTD_B) / self.nominal_resistance;
        let z4 = 2.0 * Register::RTD_B;
        let mut temperature = (libm::sqrtf(z2 + z3 * rt) + z1) / z4;

        if temperature < 0.0 {
            rt /= self.nominal_resistance;
            rt *= 100.0;
            let mut rpoly = rt;
            temperature = -242.02;
            temperature += 2.2228 * rpoly;
            rpoly *= rt;
            temperature += 2.5859e-3 * rpoly;
            rpoly *= rt;
            temperature -= 4.8260e-6 * rpoly;
            rpoly *= rt;
            temperature -= 2.8183e-8 * rpoly;
            rpoly *= rt;
            temperature += 1.5243e-10 * rpoly;
        }
        temperature.is_finite().then_some(temperature)
    }

    pub(crate) fn debug_value(&self, register: RtdResistance) -> Option<DebugValue> {
        Some(DebugValue {
            temperature: self.temperature(register)?,
            resistance: self.resistance(register),
            ratio: self.ratio(register),
        })
    }
}

/// Fault decoding from MAX31865 Rev 3 Table 7, retaining simultaneous faults.
#[derive(Default)]
pub(crate) struct FaultEvaluation;

impl FaultEvaluation {
    pub(crate) fn evaluate(&self, register: FaultStatus) -> Result<(), HardwareErr> {
        let flags = u8::from(register) & 0xfc;
        let fault = match flags {
            0 => return Ok(()),
            0x04 => HardwareErr::OverOrUnderVoltage,
            0x08 => HardwareErr::RtdinForceMinusOpen,
            0x10 => HardwareErr::RefinForceMinusOpen,
            0x20 => HardwareErr::RefinGreatherThanPoint85TimeVBias,
            0x40 => HardwareErr::RtdLowThreshold,
            0x80 => HardwareErr::RtdHighThreshold,
            _ => HardwareErr::MultipleFaults(flags.into()),
        };
        Err(fault)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn conversion(nominal_resistance: f32, reference_resistor: f32) -> RtdConversion {
        RtdConversion {
            reference_resistor,
            nominal_resistance,
        }
    }

    #[test]
    fn rtd_conversion_matches_datasheet_ratios_and_resistance() {
        let conversion = conversion(100.0, 400.0);
        for (word, ratio, resistance) in [
            (0x2000, 0.125, 50.0),
            (0x4000, 0.25, 100.0),
            (0x8000, 0.5, 200.0),
            (0xc000, 0.75, 300.0),
        ] {
            assert_eq!(conversion.ratio(word.into()), ratio);
            assert_eq!(conversion.resistance(word.into()), resistance);
        }
    }

    #[test]
    fn rtd_conversion_preserves_positive_and_negative_temperature_curves() {
        for (word, expected) in [(0x4000, 0.0), (0x6000, 130.447), (0x2690, -100.0)] {
            let value = conversion(100.0, 400.0).temperature(word.into()).unwrap();
            assert!((value - expected).abs() < 0.1, "{word:x}: {value}");
            let pt1000 = conversion(1000.0, 4000.0).temperature(word.into()).unwrap();
            assert!((pt1000 - value).abs() < 0.01);
        }
    }

    #[test]
    fn rtd_debug_value_uses_one_register_sample() {
        let value = conversion(100.0, 400.0).debug_value(0x6000.into()).unwrap();
        assert_eq!(value.ratio, 0.375);
        assert_eq!(value.resistance, 150.0);
        assert!((value.temperature - 130.447).abs() < 0.01);
    }

    #[test]
    fn rtd_temperature_rejects_nonfinite_math() {
        assert!(
            conversion(f32::MIN_POSITIVE, 400.0)
                .temperature(0x4000.into())
                .is_none()
        );
    }

    #[test]
    fn fault_evaluation_reports_every_combination_and_ignores_dont_care_bits() {
        for (value, expected) in [
            (0x04, HardwareErr::OverOrUnderVoltage),
            (0x08, HardwareErr::RtdinForceMinusOpen),
            (0x10, HardwareErr::RefinForceMinusOpen),
            (0x20, HardwareErr::RefinGreatherThanPoint85TimeVBias),
            (0x40, HardwareErr::RtdLowThreshold),
            (0x80, HardwareErr::RtdHighThreshold),
        ] {
            assert_eq!(FaultEvaluation.evaluate(value.into()), Err(expected));
        }
        for value in 0..=u8::MAX {
            let result = FaultEvaluation.evaluate(value.into());
            assert_eq!(result.is_err(), value & 0xfc != 0);
            if (value & 0xfc).count_ones() > 1 {
                assert_eq!(
                    result,
                    Err(HardwareErr::MultipleFaults((value & 0xfc).into()))
                );
            }
        }
    }
}
