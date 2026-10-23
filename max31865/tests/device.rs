use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use embedded_hal::{
    delay::DelayNs,
    spi::{ErrorKind, ErrorType, Operation, SpiDevice},
};
use max31865::{
    FaultCycle, Max31865, Numwires,
    error::{HardwareErr, Max31865Err},
};

#[derive(Clone)]
enum Step {
    Read(u8, Vec<u8>),
    Write(Vec<u8>),
    Fail(Box<Step>),
}

#[derive(Clone)]
struct MockSpi(Rc<RefCell<VecDeque<Step>>>);

impl MockSpi {
    fn new(steps: Vec<Step>) -> Self {
        Self(Rc::new(RefCell::new(steps.into())))
    }

    fn done(&self) {
        assert!(self.0.borrow().is_empty(), "unused SPI transactions");
    }
}

impl ErrorType for MockSpi {
    type Error = ErrorKind;
}

impl SpiDevice for MockSpi {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        let step = self
            .0
            .borrow_mut()
            .pop_front()
            .expect("unexpected SPI transaction");
        let (step, fail) = match step {
            Step::Fail(step) => (*step, true),
            step => (step, false),
        };
        match step {
            Step::Write(expected) => match operations {
                [Operation::Write(actual)] => assert_eq!(*actual, expected),
                _ => panic!("write must be a single SPI operation"),
            },
            Step::Read(address, expected) => match operations {
                [Operation::Write(actual), Operation::Read(bytes)] => {
                    assert_eq!(*actual, [address]);
                    assert_eq!(bytes.len(), expected.len());
                    bytes.copy_from_slice(&expected);
                }
                _ => panic!("register read must keep address and data under one CS"),
            },
            Step::Fail(_) => panic!("nested failure fixture"),
        }
        if fail {
            Err(ErrorKind::ModeFault)
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Default)]
struct Delay(Rc<RefCell<Vec<u32>>>);

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        self.0.borrow_mut().push(ns);
    }
}

fn read(address: u8, data: &[u8]) -> Step {
    Step::Read(address, data.to_vec())
}

fn write(address: u8, data: u8) -> Step {
    Step::Write(vec![address | 0x80, data])
}

fn startup() -> Vec<Step> {
    vec![
        read(0, &[0]),
        write(0, 0x10),
        read(0, &[0x10]),
        write(0, 0x90),
        read(0, &[0x90]),
        write(0, 0xd0),
        write(6, 0),
        write(5, 0),
        write(4, 0xff),
        write(3, 0xff),
        read(0, &[0xd0]),
        write(0, 0xd2),
    ]
}

fn acquisition(raw: u16) -> Vec<Step> {
    vec![
        read(0, &[0xd0]),
        write(0, 0xd2),
        read(0, &[0xd0]),
        write(0, 0xd0),
        read(0, &[0xd0]),
        write(0, 0xf0),
        read(1, &raw.to_be_bytes()),
        read(0, &[0xd0]),
        write(0, 0x50),
    ]
}

#[test]
#[allow(deprecated)]
fn regression_preserves_three_wire_startup_transactions_and_delay() {
    let spi = MockSpi::new(startup());
    let delay = Delay::default();
    let _device = Max31865::new(
        spi.clone(),
        delay.clone(),
        Numwires::MAX31865_3_WIRE,
        100.0,
        400.0,
    );
    spi.done();
    assert_eq!(*delay.0.borrow(), [20_000_000]);
}

#[test]
#[allow(deprecated)]
fn regression_preserves_successful_rtd_acquisition_and_bias_shutdown() {
    let mut steps = startup();
    steps.extend(acquisition(0x4000));
    let spi = MockSpi::new(steps);
    let delay = Delay::default();
    let mut device = Max31865::new(
        spi.clone(),
        delay.clone(),
        Numwires::MAX31865_3_WIRE,
        100.0,
        400.0,
    );
    assert_eq!(device.read_rtd().unwrap(), 8192);
    assert_eq!(*delay.0.borrow(), [20_000_000, 10_000_000, 65_000_000]);
    spi.done();
}

#[test]
#[allow(deprecated)]
fn regression_preserves_positive_temperature_conversion() {
    let mut steps = startup();
    steps.extend(acquisition(0x6000));
    let spi = MockSpi::new(steps);
    let mut device = Max31865::new(
        spi.clone(),
        Delay::default(),
        Numwires::MAX31865_3_WIRE,
        100.0,
        400.0,
    );
    assert!((device.temperature().unwrap() - 130.447).abs() < 0.01);
    spi.done();
}

fn initialized(spi: &MockSpi, delay: &Delay) -> Max31865<MockSpi, Delay> {
    Max31865::try_new(
        spi.clone(),
        delay.clone(),
        Numwires::MAX31865_3_WIRE,
        100.0,
        400.0,
    )
    .unwrap()
}

fn failing_prefix(steps: &[Step], fail_at: usize) -> Vec<Step> {
    let mut prefix = steps[..fail_at].to_vec();
    prefix.push(Step::Fail(Box::new(steps[fail_at].clone())));
    prefix
}

#[test]
fn initialization_forwards_original_error_at_every_step() {
    let steps = startup();
    for fail_at in 0..steps.len() {
        let spi = MockSpi::new(failing_prefix(&steps, fail_at));
        let result = Max31865::try_new(
            spi.clone(),
            Delay::default(),
            Numwires::MAX31865_3_WIRE,
            100.0,
            400.0,
        );
        assert!(matches!(
            result,
            Err(Max31865Err::Interface(ErrorKind::ModeFault))
        ));
        spi.done();
    }
}

#[test]
fn initialization_rejects_invalid_resistors_before_io() {
    for value in [0.0, -100.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let spi = MockSpi::new(vec![]);
        assert!(matches!(
            Max31865::try_new(
                spi.clone(),
                Delay::default(),
                Numwires::MAX31865_3_WIRE,
                value,
                400.0,
            ),
            Err(Max31865Err::InvalidNominalResistance)
        ));
        assert!(matches!(
            Max31865::try_new(
                spi.clone(),
                Delay::default(),
                Numwires::MAX31865_3_WIRE,
                100.0,
                value,
            ),
            Err(Max31865Err::InvalidReferenceResistance)
        ));
        spi.done();
    }
}

#[test]
fn settings_preserve_unrelated_fields_without_replaying_commands() {
    let mut steps = startup();
    steps.extend([
        read(0, &[0xff]),
        write(0, 0x51),
        read(0, &[0xff]),
        write(0, 0x91),
        read(0, &[0xff]),
        write(0, 0xc1),
        read(0, &[0xff]),
        write(0, 0xd3),
        read(0, &[0xff]),
        write(0, 0x91),
        write(0, 0x90),
        write(0, 0xd0),
    ]);
    let spi = MockSpi::new(steps);
    let mut device = initialized(&spi, &Delay::default());
    device.enable_bias(false).unwrap();
    device.auto_convert(false).unwrap();
    device.set_wires(Numwires::MAX31865_2_OR_4_WIRE).unwrap();
    device.clear_fault().unwrap();
    device.enable_50hz(false).unwrap();
    spi.done();
}

#[test]
fn measurement_getters_share_correct_ratio_and_resistance_scaling() {
    for getter in 0..3 {
        let mut steps = startup();
        steps.extend(acquisition(0x6000));
        let spi = MockSpi::new(steps);
        let mut device = initialized(&spi, &Delay::default());
        match getter {
            0 => assert_eq!(device.get_ratio().unwrap(), 0.375),
            1 => assert_eq!(device.get_resistance().unwrap(), 150.0),
            _ => {
                let value = device.get_debug_value().unwrap();
                assert_eq!(value.ratio, 0.375);
                assert_eq!(value.resistance, 150.0);
                assert!((value.temperature - 130.447).abs() < 0.01);
            }
        }
        spi.done();
    }
}

#[test]
fn acquisition_forwards_original_error_without_cleanup_masking_it() {
    let steps = acquisition(0x4000);
    for fail_at in 0..steps.len() {
        let mut expectations = startup();
        expectations.extend(failing_prefix(&steps, fail_at));
        let spi = MockSpi::new(expectations);
        let mut device = initialized(&spi, &Delay::default());
        assert_eq!(
            device.read_rtd(),
            Err(Max31865Err::Interface(ErrorKind::ModeFault))
        );
        spi.done();
    }
}

#[test]
fn faulted_samples_report_latched_single_combined_and_unspecified_causes() {
    for (status, error) in [
        (0x04, HardwareErr::OverOrUnderVoltage),
        (0x84, HardwareErr::MultipleFaults(0x84.into())),
        (0x00, HardwareErr::UnspecifiedRtdFault),
        (0x03, HardwareErr::UnspecifiedRtdFault),
    ] {
        let mut steps = startup();
        steps.extend(acquisition(0x4001));
        steps.push(read(7, &[status]));
        let spi = MockSpi::new(steps);
        let mut device = initialized(&spi, &Delay::default());
        assert_eq!(device.read_rtd(), Err(Max31865Err::Hardware(error)));
        spi.done();
    }
}

#[test]
fn fault_status_read_failure_is_forwarded_after_faulted_sample() {
    let mut steps = startup();
    steps.extend(acquisition(0x4001));
    steps.push(Step::Fail(Box::new(read(7, &[0x80]))));
    let spi = MockSpi::new(steps);
    let mut device = initialized(&spi, &Delay::default());
    assert_eq!(
        device.temperature(),
        Err(Max31865Err::Interface(ErrorKind::ModeFault))
    );
    spi.done();
}

#[test]
fn fifty_hertz_mode_stops_auto_for_filter_change_and_waits_max_conversion_time() {
    let mut steps = startup();
    steps.extend([
        read(0, &[0xd0]),
        write(0, 0x90),
        write(0, 0x91),
        write(0, 0xd1),
        read(0, &[0xd1]),
        write(0, 0xd3),
        read(0, &[0xd1]),
        write(0, 0xd1),
        read(0, &[0xd1]),
        write(0, 0xf1),
        read(1, &[0x40, 0]),
        read(0, &[0xd1]),
        write(0, 0x51),
    ]);
    let spi = MockSpi::new(steps);
    let delay = Delay::default();
    let mut device = initialized(&spi, &delay);
    device.enable_50hz(true).unwrap();
    assert_eq!(device.read_rtd().unwrap(), 8192);
    assert_eq!(*delay.0.borrow(), [20_000_000, 10_000_000, 66_000_000]);
    spi.done();
}

#[test]
fn filter_change_failures_stop_at_the_failing_step_without_rollback() {
    let steps = [
        read(0, &[0xd0]),
        write(0, 0x90),
        write(0, 0x91),
        write(0, 0xd1),
    ];
    for fail_at in 0..steps.len() {
        let mut expectations = startup();
        expectations.extend(failing_prefix(&steps, fail_at));
        let spi = MockSpi::new(expectations);
        let mut device = initialized(&spi, &Delay::default());
        assert_eq!(
            device.enable_50hz(true),
            Err(Max31865Err::Interface(ErrorKind::ModeFault))
        );
        spi.done();
    }
}

#[test]
fn typed_reads_and_threshold_getters_decode_big_endian_and_dont_care_bit() {
    let mut steps = startup();
    steps.extend([
        read(0, &[0xd1]),
        read(1, &[0x40, 1]),
        read(7, &[0x84]),
        read(5, &[0x12, 0x35]),
        read(3, &[0xab, 0xcd]),
        read(5, &[0x12, 0x35]),
        read(3, &[0xab, 0xcd]),
    ]);
    let spi = MockSpi::new(steps);
    let delay = Delay::default();
    let mut device = initialized(&spi, &delay);
    assert!(
        device
            .read_configuration()
            .unwrap()
            .three_wire_compensation()
    );
    let register = device.read_resistance_register().unwrap();
    assert!(register.fault());
    assert_eq!(register.count().value(), 8192);
    let status = device.read_fault_status().unwrap();
    assert!(status.rtd_high_threshold() && status.over_or_under_voltage());
    let lower = device.read_lower_threshold().unwrap();
    assert_eq!(lower.count().value(), 0x091a);
    assert!(lower.ignored());
    assert_eq!(u16::from(device.read_upper_threshold().unwrap()), 0xabcd);
    assert_eq!(device.get_lower_threshold().unwrap(), 0x1235);
    assert_eq!(device.get_upper_threshold().unwrap(), 0xabcd);
    let (_spi, returned_delay) = device.release();
    assert_eq!(*returned_delay.0.borrow(), [20_000_000]);
    spi.done();
}

#[test]
fn threshold_partial_write_failures_preserve_original_error() {
    let steps = [
        write(6, 0x34),
        write(5, 0x12),
        write(4, 0xcd),
        write(3, 0xab),
    ];
    for fail_at in 0..steps.len() {
        let mut expectations = startup();
        expectations.extend(failing_prefix(&steps, fail_at));
        let spi = MockSpi::new(expectations);
        let mut device = initialized(&spi, &Delay::default());
        assert_eq!(
            device.set_threshold(0x1234, 0xabcd),
            Err(Max31865Err::Interface(ErrorKind::ModeFault))
        );
        spi.done();
    }
}

#[test]
fn fault_cycles_preserve_wire_filter_and_do_not_replay_one_shot() {
    let mut steps = startup();
    steps.extend([
        read(7, &[0]),
        read(0, &[0xff]),
        write(0, 0x95),
        read(7, &[0]),
        read(0, &[0xff]),
        write(0, 0x99),
        read(7, &[0]),
        read(0, &[0xff]),
        write(0, 0x9d),
        read(7, &[0x20]),
    ]);
    let spi = MockSpi::new(steps);
    let delay = Delay::default();
    let mut device = initialized(&spi, &delay);
    device.read_fault(FaultCycle::None).unwrap();
    device.read_fault(FaultCycle::Auto).unwrap();
    device.read_fault(FaultCycle::ManualRun).unwrap();
    assert_eq!(
        device.read_fault(FaultCycle::ManualFinish),
        Err(Max31865Err::Hardware(
            HardwareErr::RefinGreatherThanPoint85TimeVBias
        ))
    );
    assert_eq!(*delay.0.borrow(), [20_000_000, 1_000_000, 1_000_000]);
    spi.done();
}

#[test]
fn automatic_fault_cycle_forwards_each_multistep_failure() {
    let steps = [read(0, &[0xd0]), write(0, 0x94), read(7, &[0])];
    for fail_at in 0..steps.len() {
        let mut expectations = startup();
        expectations.extend(failing_prefix(&steps, fail_at));
        let spi = MockSpi::new(expectations);
        let mut device = initialized(&spi, &Delay::default());
        assert_eq!(
            device.read_fault(FaultCycle::Auto),
            Err(Max31865Err::Interface(ErrorKind::ModeFault))
        );
        spi.done();
    }
}
