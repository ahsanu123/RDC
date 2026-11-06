use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use cst816::{Cst816s, Cst816sError, TouchPosition, TouchReading, registers::*};
use embedded_hal::i2c::{ErrorKind, ErrorType, I2c, Operation};

#[derive(Clone, Debug)]
enum Step {
    Read(u8, u8),
    Burst(u8, Vec<u8>),
    Write(u8, u8),
    Fail(Box<Step>),
}

#[derive(Clone, Debug)]
struct MockI2c(Rc<RefCell<VecDeque<Step>>>, u8);

impl MockI2c {
    fn new(steps: Vec<Step>) -> Self {
        Self::with_address(0x15, steps)
    }

    fn with_address(address: u8, steps: Vec<Step>) -> Self {
        Self(Rc::new(RefCell::new(steps.into())), address)
    }

    fn done(&self) {
        assert!(self.0.borrow().is_empty(), "unused I2C transactions");
    }
}

impl ErrorType for MockI2c {
    type Error = ErrorKind;
}

impl I2c for MockI2c {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        assert_eq!(
            address, self.1,
            "expected the configured seven-bit slave address"
        );
        let step = self
            .0
            .borrow_mut()
            .pop_front()
            .expect("unexpected I2C transaction");
        let (step, fail) = match step {
            Step::Fail(step) => (*step, true),
            step => (step, false),
        };
        match step {
            Step::Read(register, value) => match operations {
                [Operation::Write(command), Operation::Read(bytes)] => {
                    assert_eq!(*command, [register]);
                    assert_eq!(bytes.len(), 1);
                    bytes[0] = value;
                }
                _ => panic!("register read must use a single write/read transaction"),
            },
            Step::Write(register, value) => match operations {
                [Operation::Write(bytes)] => assert_eq!(*bytes, [register, value]),
                _ => panic!("register and data must be written together"),
            },
            Step::Burst(register, values) => match operations {
                [Operation::Write(command), Operation::Read(bytes)] => {
                    assert_eq!(*command, [register]);
                    assert_eq!(bytes.len(), values.len(), "exact burst length");
                    bytes.copy_from_slice(&values);
                }
                _ => panic!("burst must use one pointer write/read transaction"),
            },
            Step::Fail(_) => panic!("nested failure fixture"),
        }
        if fail { Err(ErrorKind::Bus) } else { Ok(()) }
    }
}

#[test]
fn device_reads_every_register_with_combined_high_low_values() {
    let bus = MockI2c::new(vec![
        Step::Read(0x01, 0x0B),
        Step::Read(0x02, 0x01),
        Step::Burst(0x03, vec![0xA1, 0x23]),
        Step::Burst(0x05, vec![0xB4, 0x56]),
        Step::Read(0xA7, 0xB4),
        Step::Read(0xA8, 0x12),
        Step::Read(0xA9, 0x34),
        Step::Read(0xB0, 0x89),
        Step::Read(0xB1, 0xAB),
        Step::Read(0xB2, 0xCD),
        Step::Read(0xB3, 0xEF),
        Step::Read(0xEC, 0x85),
        Step::Read(0xED, 27),
        Step::Read(0xEE, 14),
        Step::Read(0xEF, 8),
        Step::Read(0xF0, 0x12),
        Step::Read(0xF1, 0x34),
        Step::Read(0xF2, 0x56),
        Step::Read(0xF3, 0x78),
        Step::Read(0xF4, 4),
        Step::Read(0xF5, 47),
        Step::Read(0xF6, 2),
        Step::Read(0xF7, 6),
        Step::Read(0xF8, 31),
        Step::Read(0xF9, 9),
        Step::Read(0xFA, 0xF1),
        Step::Read(0xFB, 11),
        Step::Read(0xFC, 12),
        Step::Read(0xFD, 0xD3),
        Step::Read(0xFE, 0x80),
    ]);
    let mut device = Cst816s::new(bus.clone());
    assert_eq!(
        device.read_gesture_identifier().unwrap().gesture(),
        Gesture::DoubleClick
    );
    assert_eq!(
        device.read_finger_number().unwrap().count(),
        FingerCount::OneFinger
    );
    let x = device.read_x_position().unwrap();
    assert_eq!(x.coordinate().value(), 0x123);
    assert_eq!(u16::from(x), 0xA123);
    let y = device.read_y_position().unwrap();
    assert_eq!(y.coordinate().value(), 0x456);
    assert_eq!(u16::from(y), 0xB456);
    assert_eq!(u8::from(device.read_chip_identifier().unwrap()), 0xB4);
    assert_eq!(u8::from(device.read_project_identifier().unwrap()), 0x12);
    assert_eq!(u8::from(device.read_firmware_version().unwrap()), 0x34);
    assert_eq!(u16::from(device.read_bpc0().unwrap()), 0x89AB);
    assert_eq!(u16::from(device.read_bpc1().unwrap()), 0xCDEF);
    assert_eq!(u8::from(device.read_motion_mask().unwrap()), 0x85);
    assert_eq!(u8::from(device.read_interrupt_pulse_width().unwrap()), 27);
    assert_eq!(u8::from(device.read_normal_scan_period().unwrap()), 14);
    assert_eq!(u8::from(device.read_motion_sliding_angle().unwrap()), 8);
    assert_eq!(
        u16::from(device.read_low_power_scan_raw1().unwrap()),
        0x1234
    );
    assert_eq!(
        u16::from(device.read_low_power_scan_raw2().unwrap()),
        0x5678
    );
    assert_eq!(u8::from(device.read_low_power_auto_wake_time().unwrap()), 4);
    assert_eq!(
        u8::from(device.read_low_power_scan_threshold().unwrap()),
        47
    );
    assert_eq!(u8::from(device.read_low_power_scan_window().unwrap()), 2);
    assert_eq!(u8::from(device.read_low_power_scan_frequency().unwrap()), 6);
    assert_eq!(u8::from(device.read_low_power_scan_current().unwrap()), 31);
    assert_eq!(u8::from(device.read_auto_sleep_time().unwrap()), 9);
    assert_eq!(u8::from(device.read_interrupt_control().unwrap()), 0xF1);
    assert_eq!(u8::from(device.read_auto_reset().unwrap()), 11);
    assert_eq!(u8::from(device.read_long_press_time().unwrap()), 12);
    assert_eq!(u8::from(device.read_io_control().unwrap()), 0xD3);
    assert_eq!(u8::from(device.read_auto_sleep_disable().unwrap()), 0x80);
    device.release().done();
    bus.done();
}

#[test]
fn device_writes_supported_configuration_with_correct_framing() {
    let bus = MockI2c::new(vec![
        Step::Read(0xEC, 0xF8),
        Step::Write(0xEC, 0xFD),
        Step::Write(0xED, 27),
        Step::Write(0xEE, 14),
        Step::Write(0xEF, 8),
        Step::Write(0xF4, 4),
        Step::Write(0xF5, 47),
        Step::Write(0xF6, 2),
        Step::Write(0xF7, 6),
        Step::Write(0xF8, 31),
        Step::Write(0xF9, 9),
        Step::Read(0xFA, 0x0E),
        Step::Write(0xFA, 0xAF),
        Step::Write(0xFB, 11),
        Step::Write(0xFC, 12),
        Step::Read(0xFD, 0xD8),
        Step::Write(0xFD, 0xDB),
        Step::Write(0xFE, 0x80),
    ]);
    let mut device = Cst816s::new(bus.clone());
    device.write_motion_mask(MotionMask::from(0x05)).unwrap();
    device
        .write_interrupt_pulse_width(InterruptPulseWidth::from(27))
        .unwrap();
    device
        .write_normal_scan_period(NormalScanPeriod::from(14))
        .unwrap();
    device
        .write_motion_sliding_angle(MotionSlidingAngle::from(8))
        .unwrap();
    device
        .write_low_power_auto_wake_time(LowPowerAutoWakeTime::from(4))
        .unwrap();
    device
        .write_low_power_scan_threshold(LowPowerScanThreshold::from(47))
        .unwrap();
    device
        .write_low_power_scan_window(LowPowerScanWindow::from(2))
        .unwrap();
    device
        .write_low_power_scan_frequency(LowPowerScanFrequency::from(6))
        .unwrap();
    device
        .write_low_power_scan_current(LowPowerScanCurrent::from(31))
        .unwrap();
    device
        .write_auto_sleep_time(AutoSleepTime::from(9))
        .unwrap();
    device
        .write_interrupt_control(InterruptControl::from(0xA1))
        .unwrap();
    device.write_auto_reset(AutoReset::from(11)).unwrap();
    device
        .write_long_press_time(LongPressTime::from(12))
        .unwrap();
    device.write_io_control(IoControl::from(0x03)).unwrap();
    device
        .write_auto_sleep_disable(AutoSleepDisable::from(0x80))
        .unwrap();
    bus.done();
}

#[test]
fn device_custom_address_is_used_by_every_transfer_pattern() {
    let bus = MockI2c::with_address(
        0x2A,
        vec![
            Step::Read(0xA7, 0xB4),
            Step::Read(0x01, 0x0B),
            Step::Burst(0x03, vec![0xA1, 0x23]),
            Step::Write(0xED, 27),
            Step::Read(0xEC, 0xF8),
            Step::Write(0xEC, 0xFD),
            Step::Read(0xF0, 0x89),
            Step::Read(0xF1, 0xAB),
            Step::Burst(0x01, vec![0x05, 1, 0xA1, 0x23, 0xB4, 0x56]),
        ],
    );
    let mut device = Cst816s::with_address(bus.clone(), 0x2A).unwrap();
    assert_eq!(u8::from(device.read_chip_identifier().unwrap()), 0xB4);
    assert_eq!(
        device.read_gesture_identifier().unwrap().gesture(),
        Gesture::DoubleClick
    );
    assert_eq!(
        device.read_x_position().unwrap().coordinate().value(),
        0x123
    );
    device
        .write_interrupt_pulse_width(InterruptPulseWidth::from(27))
        .unwrap();
    device.write_motion_mask(MotionMask::from(0x05)).unwrap();
    assert_eq!(
        u16::from(device.read_low_power_scan_raw1().unwrap()),
        0x89AB
    );
    assert_eq!(
        device.read_touch().unwrap(),
        TouchReading {
            gesture: Gesture::SingleClick,
            position: Some(TouchPosition { x: 0x123, y: 0x456 }),
        }
    );
    bus.done();
}

#[test]
fn device_masked_writes_preserve_current_hardware_unspecified_bits() {
    for current in u8::MIN..=u8::MAX {
        let bus = MockI2c::new(vec![
            Step::Read(0xEC, current),
            Step::Write(0xEC, (current & 0xF8) | 0x02),
            Step::Read(0xFA, current),
            Step::Write(0xFA, (current & 0x0E) | 0x50),
            Step::Read(0xFD, current),
            Step::Write(0xFD, (current & 0xF8) | 0x04),
        ]);
        let mut device = Cst816s::new(bus.clone());
        device.write_motion_mask(MotionMask::from(0xFA)).unwrap();
        device
            .write_interrupt_control(InterruptControl::from(0x5E))
            .unwrap();
        device.write_io_control(IoControl::from(0xFC)).unwrap();
        bus.done();
    }
}

#[test]
fn device_pair_reads_forward_either_transfer_failure_without_a_partial_result() {
    macro_rules! check {
        ($method:ident, $high:expr, $low:expr) => {
            for fail_high in [true, false] {
                let steps = if fail_high {
                    vec![Step::Fail(Box::new(Step::Read($high, 0x12)))]
                } else {
                    vec![
                        Step::Read($high, 0x12),
                        Step::Fail(Box::new(Step::Read($low, 0x34))),
                    ]
                };
                let bus = MockI2c::new(steps);
                let mut device = Cst816s::new(bus.clone());
                assert_eq!(
                    device.$method(),
                    Err(Cst816sError::Interface(ErrorKind::Bus))
                );
                bus.done();
            }
        };
    }
    check!(read_bpc0, 0xB0, 0xB1);
    check!(read_bpc1, 0xB2, 0xB3);
    check!(read_low_power_scan_raw1, 0xF0, 0xF1);
    check!(read_low_power_scan_raw2, 0xF2, 0xF3);
}

#[test]
fn device_masked_writes_forward_read_and_write_failures() {
    macro_rules! check {
        ($method:ident, $register:ident, $address:expr, $value:expr) => {
            for fail_read in [true, false] {
                let steps = if fail_read {
                    vec![Step::Fail(Box::new(Step::Read($address, 0)))]
                } else {
                    vec![
                        Step::Read($address, 0),
                        Step::Fail(Box::new(Step::Write($address, $value))),
                    ]
                };
                let bus = MockI2c::new(steps);
                let mut device = Cst816s::new(bus.clone());
                assert_eq!(
                    device.$method($register::from($value)),
                    Err(Cst816sError::Interface(ErrorKind::Bus))
                );
                bus.done();
            }
        };
    }
    check!(write_motion_mask, MotionMask, 0xEC, 0x05);
    check!(write_interrupt_control, InterruptControl, 0xFA, 0xA1);
    check!(write_io_control, IoControl, 0xFD, 0x03);
}

#[test]
fn device_checked_reads_report_invalid_values_and_forward_interface_errors() {
    let bus = MockI2c::new(vec![
        Step::Read(0x01, 0xFF),
        Step::Read(0x02, 0x02),
        Step::Fail(Box::new(Step::Read(0x01, 0))),
        Step::Fail(Box::new(Step::Read(0x02, 0))),
        Step::Fail(Box::new(Step::Read(0xA7, 0))),
        Step::Fail(Box::new(Step::Write(0xED, 27))),
    ]);
    let mut device = Cst816s::new(bus.clone());
    assert_eq!(
        device.read_gesture_identifier(),
        Err(Cst816sError::InvalidRegisterValue {
            address: Address::GestureIdentifier,
            value: 0xFF,
        })
    );
    assert_eq!(
        device.read_finger_number(),
        Err(Cst816sError::InvalidRegisterValue {
            address: Address::FingerNumber,
            value: 0x02,
        })
    );
    assert_eq!(
        device.read_gesture_identifier(),
        Err(Cst816sError::Interface(ErrorKind::Bus))
    );
    assert_eq!(
        device.read_finger_number(),
        Err(Cst816sError::Interface(ErrorKind::Bus))
    );
    assert_eq!(
        device.read_chip_identifier(),
        Err(Cst816sError::Interface(ErrorKind::Bus))
    );
    assert_eq!(
        device.write_interrupt_pulse_width(InterruptPulseWidth::from(27)),
        Err(Cst816sError::Interface(ErrorKind::Bus))
    );
    bus.done();
}

#[test]
fn device_construction_and_release_do_not_access_the_bus() {
    let bus = MockI2c::new(vec![]);
    let returned = Cst816s::new(bus.clone()).release();
    assert!(Rc::ptr_eq(&bus.0, &returned.0));
    returned.done();

    let custom_bus = MockI2c::with_address(0x2A, vec![]);
    let returned = Cst816s::with_address(custom_bus.clone(), 0x2A)
        .unwrap()
        .release();
    assert!(Rc::ptr_eq(&custom_bus.0, &returned.0));
    returned.done();
}

#[test]
fn device_writes_validate_every_raw_setting_before_any_transfer() {
    macro_rules! check {
        ($method:ident, $register:ident, $low:expr, $high:expr) => {
            for raw in u8::MIN..=u8::MAX {
                let address = $register::ADDRESS;
                let valid = ($low..=$high).contains(&raw);
                let bus = MockI2c::new(if valid {
                    vec![Step::Write(address as u8, raw)]
                } else {
                    vec![]
                });
                let mut device = Cst816s::new(bus.clone());
                let result = device.$method($register::from(raw));
                if valid {
                    assert_eq!(result, Ok(()), "{} rejected {raw}", stringify!($method));
                } else {
                    assert_eq!(
                        result,
                        Err(Cst816sError::InvalidRegisterValue {
                            address,
                            value: raw
                        }),
                        "{} accepted {raw}",
                        stringify!($method)
                    );
                }
                bus.done();
            }
        };
    }
    check!(write_normal_scan_period, NormalScanPeriod, 1, 30);
    check!(write_interrupt_pulse_width, InterruptPulseWidth, 1, 200);
    check!(write_low_power_scan_window, LowPowerScanWindow, 0, 3);
    check!(write_low_power_auto_wake_time, LowPowerAutoWakeTime, 1, 5);
    check!(
        write_low_power_scan_threshold,
        LowPowerScanThreshold,
        1,
        255
    );
    check!(
        write_low_power_scan_frequency,
        LowPowerScanFrequency,
        1,
        255
    );
    check!(write_low_power_scan_current, LowPowerScanCurrent, 1, 255);
}

#[test]
fn device_checked_address_rejects_high_bit_and_returns_owned_bus_without_io() {
    for address in 0..=u8::MAX {
        let bus = MockI2c::with_address(address, vec![]);
        let returned = match Cst816s::with_address(bus.clone(), address) {
            Ok(device) => {
                assert!(address <= 0x7F);
                device.release()
            }
            Err(error) => {
                assert!(address >= 0x80);
                assert_eq!(error.address, address);
                error.i2c
            }
        };
        assert!(Rc::ptr_eq(&bus.0, &returned.0));
        returned.done();
    }
}

#[test]
fn device_invalid_settings_are_lossless_on_read() {
    let bus = MockI2c::new(vec![
        Step::Read(0xEE, 255),
        Step::Read(0xED, 0),
        Step::Read(0xF4, 255),
        Step::Read(0xF5, 0),
        Step::Read(0xF6, 255),
        Step::Read(0xF7, 0),
        Step::Read(0xF8, 0),
    ]);
    let mut device = Cst816s::new(bus.clone());
    assert_eq!(u8::from(device.read_normal_scan_period().unwrap()), 255);
    assert_eq!(u8::from(device.read_interrupt_pulse_width().unwrap()), 0);
    assert_eq!(
        u8::from(device.read_low_power_auto_wake_time().unwrap()),
        255
    );
    assert_eq!(u8::from(device.read_low_power_scan_threshold().unwrap()), 0);
    assert_eq!(u8::from(device.read_low_power_scan_window().unwrap()), 255);
    assert_eq!(u8::from(device.read_low_power_scan_frequency().unwrap()), 0);
    assert_eq!(u8::from(device.read_low_power_scan_current().unwrap()), 0);
    bus.done();
}

#[test]
#[allow(deprecated)]
fn device_compatibility_alias_retains_checked_construction() {
    let bus = MockI2c::new(vec![]);
    let error = cst816::Cst816x::with_address(bus.clone(), 0x80)
        .err()
        .unwrap();
    assert_eq!(error.address, 0x80);
    assert!(Rc::ptr_eq(&bus.0, &error.i2c.0));
    cst816::device::Cst816x::new(bus.clone()).release().done();
    bus.done();
}

#[test]
fn device_touch_reads_exact_window_and_discards_only_coordinate_high_nibbles() {
    for (raw, gesture) in [
        (0, Gesture::NoGesture),
        (1, Gesture::SlideUp),
        (2, Gesture::SlideDown),
        (3, Gesture::SlideLeft),
        (4, Gesture::SlideRight),
        (5, Gesture::SingleClick),
        (0x0B, Gesture::DoubleClick),
        (0x0C, Gesture::LongPress),
    ] {
        for fingers in [0, 1] {
            let bus = MockI2c::new(vec![Step::Burst(
                0x01,
                vec![raw, fingers, 0xA1, 0x23, 0xFF, 0xFF],
            )]);
            let mut device = Cst816s::new(bus.clone());
            assert_eq!(
                device.read_touch().unwrap(),
                TouchReading {
                    gesture,
                    position: if fingers == 0 {
                        None
                    } else {
                        Some(TouchPosition { x: 0x123, y: 0xFFF })
                    },
                }
            );
            bus.done();
        }
    }
}

#[test]
fn device_touch_rejects_undocumented_gesture_and_whole_finger_count_bytes() {
    for (data, address, value) in [
        (vec![0xFF, 1, 0, 0, 0, 0], Address::GestureIdentifier, 0xFF),
        (vec![0, 2, 0, 0, 0, 0], Address::FingerNumber, 2),
        (vec![0, 0x11, 0, 0, 0, 0], Address::FingerNumber, 0x11),
        (vec![0, 0xFF, 0, 0, 0, 0], Address::FingerNumber, 0xFF),
    ] {
        let bus = MockI2c::new(vec![Step::Burst(0x01, data)]);
        let mut device = Cst816s::new(bus.clone());
        assert_eq!(
            device.read_touch(),
            Err(Cst816sError::InvalidRegisterValue { address, value })
        );
        bus.done();
    }
}

#[test]
fn device_touch_reports_origin_when_a_finger_is_present() {
    let bus = MockI2c::new(vec![Step::Burst(0x01, vec![0, 1, 0, 0, 0, 0])]);
    let mut device = Cst816s::new(bus.clone());
    assert_eq!(
        device.read_touch().unwrap(),
        TouchReading {
            gesture: Gesture::NoGesture,
            position: Some(TouchPosition { x: 0, y: 0 }),
        }
    );
    bus.done();
}

#[test]
fn device_touch_window_and_coordinate_reads_forward_bus_failure_without_retry() {
    macro_rules! check {
        ($method:ident, $address:expr, $bytes:expr) => {
            let bus = MockI2c::new(vec![Step::Fail(Box::new(Step::Burst($address, $bytes)))]);
            let mut device = Cst816s::new(bus.clone());
            assert_eq!(
                device.$method(),
                Err(Cst816sError::Interface(ErrorKind::Bus))
            );
            bus.done();
        };
    }
    check!(read_touch, 0x01, vec![0, 1, 0, 0, 0, 0]);
    check!(read_x_position, 0x03, vec![0, 0]);
    check!(read_y_position, 0x05, vec![0, 0]);
}

#[test]
fn device_timing_methods_read_then_process_and_forward_bus_errors() {
    macro_rules! check {
        ($method:ident, $address:expr, $raw:expr, $expected:expr) => {
            let bus = MockI2c::new(vec![
                Step::Read($address, $raw),
                Step::Fail(Box::new(Step::Read($address, 0))),
            ]);
            let mut device = Cst816s::new(bus.clone());
            assert_eq!(device.$method().unwrap(), $expected);
            assert_eq!(
                device.$method(),
                Err(Cst816sError::Interface(ErrorKind::Bus))
            );
            bus.done();
        };
    }
    check!(read_interrupt_pulse_width_microseconds, 0xED, 200, 20000);
    check!(read_normal_scan_period_milliseconds, 0xEE, 30, 300);
    check!(read_low_power_auto_wake_time_seconds, 0xF4, 5, 300);
}
