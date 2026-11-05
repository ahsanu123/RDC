use bilge::prelude::*;
use cst816::registers::*;

#[test]
fn register_gesture_decoding_rejects_undocumented_codes() {
    let gestures = [
        (0x00, Gesture::NoGesture),
        (0x01, Gesture::SlideUp),
        (0x02, Gesture::SlideDown),
        (0x03, Gesture::SlideLeft),
        (0x04, Gesture::SlideRight),
        (0x05, Gesture::SingleClick),
        (0x0B, Gesture::DoubleClick),
        (0x0C, Gesture::LongPress),
    ];
    for raw in u8::MIN..=u8::MAX {
        let result = GestureIdentifier::try_from(raw);
        match gestures.iter().find(|(code, _)| *code == raw) {
            Some((_, gesture)) => {
                let register = result.unwrap();
                assert_eq!(register.gesture(), *gesture);
                assert_eq!(u8::from(register), raw);
            }
            None => assert!(result.is_err(), "accepted gesture code {raw:#04x}"),
        }
    }
}

#[test]
fn register_finger_decoding_rejects_undocumented_counts() {
    assert_eq!(
        FingerNumber::try_from(0).unwrap().count(),
        FingerCount::NoFinger
    );
    assert_eq!(
        FingerNumber::try_from(1).unwrap().count(),
        FingerCount::OneFinger
    );
    for raw in 2..=u8::MAX {
        assert!(FingerNumber::try_from(raw).is_err());
    }
}

#[test]
fn register_coordinate_updates_preserve_unspecified_high_nibble() {
    for raw in u8::MIN..=u8::MAX {
        let mut x = XPositionHigh::from(raw);
        let mut y = YPositionHigh::from(raw);
        assert_eq!(x.coordinate().value(), raw & 0x0F);
        assert_eq!(y.coordinate().value(), raw & 0x0F);
        x.set_coordinate(u4::new(5));
        y.set_coordinate(u4::new(10));
        assert_eq!(u8::from(x), (raw & 0xF0) | 5);
        assert_eq!(u8::from(y), (raw & 0xF0) | 10);
    }
}

#[test]
fn register_motion_updates_preserve_unspecified_bits() {
    for raw in u8::MIN..=u8::MAX {
        let mut register = MotionMask::from(raw);
        assert_eq!(register.double_click_enabled(), raw & 0x01 != 0);
        assert_eq!(register.continuous_up_down_enabled(), raw & 0x02 != 0);
        assert_eq!(register.continuous_left_right_enabled(), raw & 0x04 != 0);
        register.set_double_click_enabled(true);
        register.set_continuous_up_down_enabled(false);
        register.set_continuous_left_right_enabled(true);
        assert_eq!(u8::from(register), (raw & 0xF8) | 0x05);
    }
}

#[test]
fn register_interrupt_updates_follow_field_descriptions_and_preserve_bits() {
    for raw in u8::MIN..=u8::MAX {
        let mut register = InterruptControl::from(raw);
        assert_eq!(register.single_pulse_on_long_press(), raw & 0x01 != 0);
        assert_eq!(register.gesture_interrupt_enabled(), raw & 0x10 != 0);
        assert_eq!(register.touch_change_interrupt_enabled(), raw & 0x20 != 0);
        assert_eq!(register.touch_interrupt_enabled(), raw & 0x40 != 0);
        assert_eq!(register.interrupt_test_enabled(), raw & 0x80 != 0);
        register.set_single_pulse_on_long_press(true);
        register.set_gesture_interrupt_enabled(false);
        register.set_touch_change_interrupt_enabled(true);
        register.set_touch_interrupt_enabled(false);
        register.set_interrupt_test_enabled(true);
        assert_eq!(u8::from(register), (raw & 0x0E) | 0xA1);
    }
}

#[test]
fn register_io_updates_preserve_unspecified_bits() {
    for raw in u8::MIN..=u8::MAX {
        let mut register = IoControl::from(raw);
        assert_eq!(
            register.voltage(),
            if raw & 1 == 0 {
                IoVoltage::Vdd
            } else {
                IoVoltage::Volts1_8
            }
        );
        assert_eq!(
            register.two_wire_driver_mode(),
            if raw & 2 == 0 {
                TwoWireDriverMode::PullUpResistor
            } else {
                TwoWireDriverMode::OpenDrain
            }
        );
        assert_eq!(register.irq_soft_reset_enabled(), raw & 4 != 0);
        register.set_voltage(IoVoltage::Volts1_8);
        register.set_two_wire_driver_mode(TwoWireDriverMode::OpenDrain);
        register.set_irq_soft_reset_enabled(false);
        assert_eq!(u8::from(register), (raw & 0xF8) | 0x03);
    }
}
