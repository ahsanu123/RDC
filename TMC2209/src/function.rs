use crate::{
    DeviceError,
    register::{ChopperConfiguration, DriverStatus, MicrostepCurrent, PwmScaling, StepInterval},
};

/// Pure processing of register read results; physical user parameters stay on the device.
/// Datasheet rev. 1.09, sections 5.2, 5.4, 5.5 and 9.
#[derive(Default)]
pub(crate) struct RegisterProcessing;

fn signed_nine_bits(bits: u16) -> i16 {
    ((bits << 7) as i16) >> 7
}

impl RegisterProcessing {
    pub(crate) fn microstep_count<E>(
        &self,
        register: ChopperConfiguration,
    ) -> Result<u16, DeviceError<E>> {
        let resolution = register.microstep_resolution().value();
        if resolution > 8 {
            return Err(DeviceError::InvalidMicrostepResolution { actual: resolution }.reported());
        }
        Ok(256_u16 >> resolution)
    }

    pub(crate) fn phase_currents(&self, register: MicrostepCurrent) -> (i16, i16) {
        (
            signed_nine_bits(register.phase_a_bits().value()),
            signed_nine_bits(register.phase_b_bits().value()),
        )
    }

    pub(crate) fn pwm_correction(&self, register: PwmScaling) -> i16 {
        signed_nine_bits(register.automatic_correction_bits().value())
    }

    pub(crate) fn step_clock_period<E>(
        &self,
        register: StepInterval,
    ) -> Result<u32, DeviceError<E>> {
        let period = register.clock_cycles().value();
        if period == 0 || period == 0x000f_ffff {
            return Err(DeviceError::StepIntervalUnavailable.reported());
        }
        Ok(period)
    }

    pub(crate) fn rms_sense_voltage(
        &self,
        chopper: ChopperConfiguration,
        status: DriverStatus,
    ) -> f32 {
        let full_scale = if chopper.high_sense_sensitivity() {
            0.180
        } else {
            0.325
        };
        (f32::from(status.actual_current_scale().value()) + 1.0) / 32.0
            * full_scale
            * core::f32::consts::FRAC_1_SQRT_2
    }
}

#[cfg(test)]
mod tests {
    use core::convert::Infallible;

    use super::*;

    #[test]
    fn test_microstep_count_all_valid_encodings_and_reserved_values() {
        for resolution in 0..=8 {
            assert_eq!(
                RegisterProcessing
                    .microstep_count::<Infallible>(ChopperConfiguration::from(resolution << 24)),
                Ok(256 >> resolution)
            );
        }
        for resolution in 9..=15 {
            assert_eq!(
                RegisterProcessing
                    .microstep_count::<Infallible>(ChopperConfiguration::from(resolution << 24)),
                Err(DeviceError::InvalidMicrostepResolution {
                    actual: resolution as u8
                })
            );
        }
    }

    #[test]
    fn test_phase_currents_signed_boundaries_and_phase_order() {
        assert_eq!(
            RegisterProcessing.phase_currents(MicrostepCurrent::from((0x101 << 16) | 0xff)),
            (-255, 255)
        );
        assert_eq!(
            RegisterProcessing.phase_currents(MicrostepCurrent::from((0x100 << 16) | 0x1ff)),
            (-256, -1)
        );
        assert_eq!(
            RegisterProcessing.phase_currents(MicrostepCurrent::from(0)),
            (0, 0)
        );
    }

    #[test]
    fn test_pwm_correction_signed_boundaries() {
        for (raw, expected) in [
            (0, 0),
            (0xff, 255),
            (0x100, -256),
            (0x101, -255),
            (0x1ff, -1),
        ] {
            assert_eq!(
                RegisterProcessing.pwm_correction(PwmScaling::from(raw << 16)),
                expected
            );
        }
    }

    #[test]
    fn test_step_clock_period_rejects_zero_and_overflow() {
        for raw in [0, 0x000f_ffff] {
            assert_eq!(
                RegisterProcessing.step_clock_period::<Infallible>(StepInterval::from(raw)),
                Err(DeviceError::StepIntervalUnavailable)
            );
        }
        assert_eq!(
            RegisterProcessing.step_clock_period::<Infallible>(StepInterval::from(1200)),
            Ok(1200)
        );
    }

    #[test]
    fn test_rms_sense_voltage_current_scale_and_sensitivity() {
        let status = DriverStatus::from(31 << 16);
        let normal = RegisterProcessing.rms_sense_voltage(ChopperConfiguration::from(0), status);
        let sensitive =
            RegisterProcessing.rms_sense_voltage(ChopperConfiguration::from(1 << 17), status);
        let minimum = RegisterProcessing
            .rms_sense_voltage(ChopperConfiguration::from(0), DriverStatus::from(0));
        assert!((normal - 0.229_809_7).abs() < 0.000_001);
        assert!((sensitive - 0.127_279_2).abs() < 0.000_001);
        assert!((minimum * 32.0 - normal).abs() < 0.000_001);
    }
}
