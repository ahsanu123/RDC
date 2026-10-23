use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use embedded_hal::{
    delay::DelayNs,
    spi::{ErrorKind, ErrorType, Operation, SpiDevice},
};
use max31865::{Max31865, Numwires};

#[derive(Clone)]
enum Step {
    Read(u8, Vec<u8>),
    Write(Vec<u8>),
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
        match self
            .0
            .borrow_mut()
            .pop_front()
            .expect("unexpected SPI transaction")
        {
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
        }
        Ok(())
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
