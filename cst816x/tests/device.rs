use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use cst816::{Cst816sError, Cst816x, registers::*};
use embedded_hal::i2c::{ErrorKind, ErrorType, I2c, Operation};

#[derive(Clone)]
enum Step {
    Read(u8, u8),
    Write(u8, u8),
    Fail(Box<Step>),
}

#[derive(Clone)]
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
        Step::Read(0x03, 0xA1),
        Step::Read(0x04, 0x23),
        Step::Read(0x05, 0xB4),
        Step::Read(0x06, 0x56),
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
    let mut device = Cst816x::new(bus.clone());
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
fn device_writes_configuration_and_combined_pairs_with_correct_framing() {
    let bus = MockI2c::new(vec![
        Step::Write(0xB0, 0x12),
        Step::Write(0xB1, 0x34),
        Step::Write(0xB2, 0xFE),
        Step::Write(0xB3, 0xDC),
        Step::Read(0xEC, 0xF8),
        Step::Write(0xEC, 0xFD),
        Step::Write(0xED, 27),
        Step::Write(0xEE, 14),
        Step::Write(0xEF, 8),
        Step::Write(0xF0, 0x89),
        Step::Write(0xF1, 0xAB),
        Step::Write(0xF2, 0xCD),
        Step::Write(0xF3, 0xEF),
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
    let mut device = Cst816x::new(bus.clone());
    device.write_bpc0(Bpc0::from(0x1234)).unwrap();
    device.write_bpc1(Bpc1::from(0xFEDC)).unwrap();
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
        .write_low_power_scan_raw1(LowPowerScanRaw1::from(0x89AB))
        .unwrap();
    device
        .write_low_power_scan_raw2(LowPowerScanRaw2::from(0xCDEF))
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
            Step::Read(0x03, 0xA1),
            Step::Read(0x04, 0x23),
            Step::Write(0xED, 27),
            Step::Read(0xEC, 0xF8),
            Step::Write(0xEC, 0xFD),
            Step::Write(0xF0, 0x89),
            Step::Write(0xF1, 0xAB),
        ],
    );
    let mut device = Cst816x::with_address(bus.clone(), 0x2A);
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
    device
        .write_low_power_scan_raw1(LowPowerScanRaw1::from(0x89AB))
        .unwrap();
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
        let mut device = Cst816x::new(bus.clone());
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
                let mut device = Cst816x::new(bus.clone());
                assert_eq!(
                    device.$method(),
                    Err(Cst816sError::Interface(ErrorKind::Bus))
                );
                bus.done();
            }
        };
    }
    check!(read_x_position, 0x03, 0x04);
    check!(read_y_position, 0x05, 0x06);
    check!(read_bpc0, 0xB0, 0xB1);
    check!(read_bpc1, 0xB2, 0xB3);
    check!(read_low_power_scan_raw1, 0xF0, 0xF1);
    check!(read_low_power_scan_raw2, 0xF2, 0xF3);
}

#[test]
fn device_pair_writes_stop_on_failure_without_retry_or_rollback() {
    macro_rules! check {
        ($method:ident, $register:ident, $high:expr, $low:expr) => {
            for fail_high in [true, false] {
                let steps = if fail_high {
                    vec![Step::Fail(Box::new(Step::Write($high, 0x12)))]
                } else {
                    vec![
                        Step::Write($high, 0x12),
                        Step::Fail(Box::new(Step::Write($low, 0x34))),
                    ]
                };
                let bus = MockI2c::new(steps);
                let mut device = Cst816x::new(bus.clone());
                assert_eq!(
                    device.$method($register::from(0x1234)),
                    Err(Cst816sError::Interface(ErrorKind::Bus))
                );
                bus.done();
            }
        };
    }
    check!(write_bpc0, Bpc0, 0xB0, 0xB1);
    check!(write_bpc1, Bpc1, 0xB2, 0xB3);
    check!(write_low_power_scan_raw1, LowPowerScanRaw1, 0xF0, 0xF1);
    check!(write_low_power_scan_raw2, LowPowerScanRaw2, 0xF2, 0xF3);
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
                let mut device = Cst816x::new(bus.clone());
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
    let mut device = Cst816x::new(bus.clone());
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
    let returned = Cst816x::new(bus.clone()).release();
    assert!(Rc::ptr_eq(&bus.0, &returned.0));
    returned.done();

    let custom_bus = MockI2c::with_address(0x2A, vec![]);
    let returned = Cst816x::with_address(custom_bus.clone(), 0x2A).release();
    assert!(Rc::ptr_eq(&custom_bus.0, &returned.0));
    returned.done();
}
