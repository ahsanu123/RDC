use crate::register::{MagneticOutputRegisters, OutputByte};
use crate::{MagneticField, MagneticMeasurement};

const NULL_FIELD_COUNTS: i32 = 131_072;
const COUNTS_PER_GAUSS: f32 = 16_384.0;

/// Rev A, pages 2 and 13: unsigned 18-bit magnetic conversion.
#[derive(Default)]
pub(crate) struct MagneticConversion;

impl MagneticConversion {
    pub(crate) fn convert(&self, registers: MagneticOutputRegisters) -> MagneticMeasurement {
        let low = registers.least_significant_bits;
        let raw = [
            (u32::from(registers.x_high.value()) << 10)
                | (u32::from(registers.x_low.value()) << 2)
                | u32::from(low.x().value()),
            (u32::from(registers.y_high.value()) << 10)
                | (u32::from(registers.y_low.value()) << 2)
                | u32::from(low.y().value()),
            (u32::from(registers.z_high.value()) << 10)
                | (u32::from(registers.z_low.value()) << 2)
                | u32::from(low.z().value()),
        ];
        MagneticMeasurement {
            raw,
            gauss: MagneticField::from_array(
                raw.map(|count| (count as i32 - NULL_FIELD_COUNTS) as f32 / COUNTS_PER_GAUSS),
            ),
        }
    }
}

/// Rev A, page 14: nominal temperature transfer function.
#[derive(Default)]
pub(crate) struct TemperatureConversion;

impl TemperatureConversion {
    pub(crate) fn convert(&self, register: OutputByte) -> f32 {
        -75.0 + 0.8 * f32::from(register.value())
    }
}

/// Rev A, page 18: SET/RESET bridge offset cancellation.
#[derive(Default)]
pub(crate) struct OffsetCompensation;

impl OffsetCompensation {
    pub(crate) fn convert(
        &self,
        set: MagneticOutputRegisters,
        reset: MagneticOutputRegisters,
    ) -> MagneticField {
        let set = MagneticConversion.convert(set).raw;
        let reset = MagneticConversion.convert(reset).raw;
        MagneticField::from_array(core::array::from_fn(|axis| {
            (set[axis] as i32 - reset[axis] as i32) as f32 / (2.0 * COUNTS_PER_GAUSS)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magnetic_conversion_decodes_all_axis_low_bits() {
        let result = MagneticConversion.convert([0x80, 0, 0x80, 0, 0x80, 0, 0x6c].into());
        assert_eq!(result.raw, [131_073, 131_074, 131_075]);
        assert_eq!(result.gauss.x, 1.0 / 16384.0);
        assert_eq!(result.gauss.y, 2.0 / 16384.0);
        assert_eq!(result.gauss.z, 3.0 / 16384.0);
    }

    #[test]
    fn magnetic_conversion_handles_full_range_and_zero_field() {
        let result = MagneticConversion.convert([0, 0, 0xff, 0xff, 0x80, 0, 0x30].into());
        assert_eq!(result.raw, [0, 262_143, 131_072]);
        assert_eq!(result.gauss.x, -8.0);
        assert_eq!(result.gauss.y, 8.0 - 1.0 / 16384.0);
        assert_eq!(result.gauss.z, 0.0);
    }

    #[test]
    fn temperature_conversion_handles_endpoints_and_room_temperature() {
        for (raw, expected) in [(0, -75.0), (125, 25.0), (250, 125.0), (255, 129.0)] {
            assert!((TemperatureConversion.convert(raw.into()) - expected).abs() < 0.0001);
        }
    }

    #[test]
    fn offset_compensation_preserves_sign_and_half_counts() {
        let set = [0x90, 0, 0x70, 0, 0x80, 0, 0x40].into();
        let reset = [0x70, 0, 0x90, 0, 0x80, 0, 0x04].into();
        let result = OffsetCompensation.convert(set, reset);
        assert_eq!(result.x, 1.0 + 1.0 / 32768.0);
        assert_eq!(result.y, -1.0);
        assert_eq!(result.z, -1.0 / 32768.0);
    }
}
