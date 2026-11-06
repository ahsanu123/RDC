use crate::{
    device::{TouchPosition, TouchReading},
    register::{
        FingerCount, FingerNumber, GestureIdentifier, InterruptPulseWidth, LowPowerAutoWakeTime,
        NormalScanPeriod, XPosition, YPosition,
    },
};

/// Register-derived touch processing (CST816S register declaration, page 1).
#[derive(Default)]
pub(crate) struct TouchFunction;

impl TouchFunction {
    pub(crate) fn process(
        &self,
        gesture: GestureIdentifier,
        fingers: FingerNumber,
        x: XPosition,
        y: YPosition,
    ) -> TouchReading {
        TouchReading {
            gesture: gesture.gesture(),
            position: match fingers.count() {
                FingerCount::NoFinger => None,
                FingerCount::OneFinger => Some(TouchPosition {
                    x: x.coordinate().value(),
                    y: y.coordinate().value(),
                }),
            },
        }
    }
}

/// Integer unit conversions (CST816S register declaration, pages 3-4).
#[derive(Default)]
pub(crate) struct TimingFunction;

impl TimingFunction {
    pub(crate) fn interrupt_pulse_microseconds(&self, value: InterruptPulseWidth) -> u32 {
        u32::from(value.tenths_of_millisecond()) * 100
    }

    pub(crate) fn normal_scan_milliseconds(&self, value: NormalScanPeriod) -> u16 {
        u16::from(value.tens_of_milliseconds()) * 10
    }

    pub(crate) fn auto_wake_seconds(&self, value: LowPowerAutoWakeTime) -> u16 {
        u16::from(value.minutes()) * 60
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::Gesture;

    #[test]
    fn function_touch_masks_coordinates_and_keeps_gesture_independent_of_contact() {
        let function = TouchFunction;
        let gesture = GestureIdentifier::try_from(0x0B).unwrap();
        let x = XPosition::from(0xA123);
        let y = YPosition::from(0xFFFF);
        assert_eq!(
            function.process(gesture, FingerNumber::try_from(1).unwrap(), x, y),
            TouchReading {
                gesture: Gesture::DoubleClick,
                position: Some(TouchPosition { x: 0x123, y: 0xFFF }),
            }
        );
        assert_eq!(
            function.process(gesture, FingerNumber::try_from(0).unwrap(), x, y),
            TouchReading {
                gesture: Gesture::DoubleClick,
                position: None
            }
        );
    }

    #[test]
    fn function_timing_converts_units_without_overflow_or_rounding() {
        let function = TimingFunction;
        for (raw, microseconds, milliseconds, seconds) in [
            (0, 0, 0, 0),
            (1, 100, 10, 60),
            (30, 3000, 300, 1800),
            (200, 20000, 2000, 12000),
            (255, 25500, 2550, 15300),
        ] {
            assert_eq!(
                function.interrupt_pulse_microseconds(InterruptPulseWidth::from(raw)),
                microseconds
            );
            assert_eq!(
                function.normal_scan_milliseconds(NormalScanPeriod::from(raw)),
                milliseconds
            );
            assert_eq!(
                function.auto_wake_seconds(LowPowerAutoWakeTime::from(raw)),
                seconds
            );
        }
    }
}
