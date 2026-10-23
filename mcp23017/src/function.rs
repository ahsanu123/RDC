use crate::register::{GeneralPurposeInputOutput, InterruptCapture, InterruptFlags, OutputLatch};

/// Pure conversions of typed register read results; device methods own bus I/O.
/// Datasheet: DS20001952D, sections 3.5.8-3.5.11, per-port flags, capture, GPIO and latches.
#[derive(Default)]
pub(crate) struct PortValues;

impl PortValues {
    pub(crate) fn combine_input_levels(
        &self,
        a: GeneralPurposeInputOutput,
        b: GeneralPurposeInputOutput,
    ) -> u16 {
        u16::from_le_bytes([a.into(), b.into()])
    }

    pub(crate) fn combine_captured_levels(&self, a: InterruptCapture, b: InterruptCapture) -> u16 {
        u16::from_le_bytes([a.into(), b.into()])
    }

    pub(crate) fn combine_interrupt_flags(&self, a: InterruptFlags, b: InterruptFlags) -> u16 {
        u16::from_le_bytes([a.into(), b.into()])
    }

    pub(crate) fn combine_output_latches(&self, a: OutputLatch, b: OutputLatch) -> u16 {
        u16::from_le_bytes([a.into(), b.into()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: [(u8, u8, u16); 4] = [
        (0x00, 0x00, 0x0000),
        (0xff, 0xff, 0xffff),
        (0x81, 0x42, 0x4281),
        (0x00, 0x80, 0x8000),
    ];

    #[test]
    fn test_combine_input_levels_port_order_and_boundaries() {
        for (a, b, expected) in CASES {
            assert_eq!(
                PortValues.combine_input_levels(a.into(), b.into()),
                expected
            );
        }
    }

    #[test]
    fn test_combine_captured_levels_port_order_and_boundaries() {
        for (a, b, expected) in CASES {
            assert_eq!(
                PortValues.combine_captured_levels(a.into(), b.into()),
                expected
            );
        }
    }

    #[test]
    fn test_combine_interrupt_flags_port_order_and_boundaries() {
        for (a, b, expected) in CASES {
            assert_eq!(
                PortValues.combine_interrupt_flags(a.into(), b.into()),
                expected
            );
        }
    }

    #[test]
    fn test_combine_output_latches_port_order_and_boundaries() {
        for (a, b, expected) in CASES {
            assert_eq!(
                PortValues.combine_output_latches(a.into(), b.into()),
                expected
            );
        }
    }
}
