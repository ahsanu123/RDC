use std::collections::VecDeque;

use embedded_hal::spi::{ErrorKind, ErrorType, Operation, SpiDevice};
use ws2812::{Error, RGB, RgbColor, Ws2812};

struct ExpectedTransaction {
    writes: Vec<Vec<u8>>,
    fail_at: Option<usize>,
}

impl ExpectedTransaction {
    fn frame(payload: &[u8]) -> Self {
        Self {
            writes: vec![vec![0; 128], payload.to_vec(), vec![0; 128]],
            fail_at: None,
        }
    }

    fn latch() -> Self {
        Self {
            writes: vec![vec![0; 128]],
            fail_at: None,
        }
    }
}

struct MockSpi {
    expected: VecDeque<ExpectedTransaction>,
    transactions: usize,
}

impl MockSpi {
    fn new(expected: impl IntoIterator<Item = ExpectedTransaction>) -> Self {
        Self {
            expected: expected.into_iter().collect(),
            transactions: 0,
        }
    }

    fn execute(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), ErrorKind> {
        let expected = self.expected.pop_front().expect("unexpected transaction");
        self.transactions += 1;
        assert_eq!(operations.len(), expected.writes.len());
        for (index, (operation, bytes)) in operations.iter().zip(expected.writes).enumerate() {
            match operation {
                Operation::Write(actual) => assert_eq!(*actual, bytes),
                _ => panic!("unexpected SPI operation"),
            }
            if expected.fail_at == Some(index) {
                return Err(ErrorKind::ModeFault);
            }
        }
        Ok(())
    }

    fn done(self, transactions: usize) {
        assert!(self.expected.is_empty(), "unused transactions");
        assert_eq!(self.transactions, transactions);
    }
}

impl ErrorType for MockSpi {
    type Error = ErrorKind;
}

impl SpiDevice for MockSpi {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        self.execute(operations)
    }
}

#[cfg(feature = "async")]
impl embedded_hal_async::spi::SpiDevice for MockSpi {
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        self.execute(operations)
    }
}

// Hand-encoded, asymmetric channel values detect bit reversal and channel overlap.
const COLOR: RgbColor = RgbColor::new(0x01, 0x80, 0x55);
const PAYLOAD: [u8; 12] = [
    0xc8, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x8c, 0x8c, 0x8c, 0x8c, 0x8c,
];

#[test]
fn write_color_sends_grb_msb_first_with_reset_prefix_and_suffix() {
    let spi = MockSpi::new([ExpectedTransaction::frame(&PAYLOAD)]);
    let mut leds = Ws2812::new(spi);
    leds.write_color(&RGB::new(0x01, 0x80, 0x55)).unwrap();
    leds.release().done(1);
}

#[test]
fn write_colors_sends_entire_chain_in_one_payload_operation() {
    let mut payload = PAYLOAD.to_vec();
    payload.extend_from_slice(&[0x88; 12]);
    payload.extend_from_slice(&[0xcc; 12]);
    let spi = MockSpi::new([ExpectedTransaction::frame(&payload)]);
    let mut leds = Ws2812::new(spi);
    leds.write_colors(&[COLOR, RgbColor::default(), RgbColor::new(255, 255, 255)])
        .unwrap();
    leds.release().done(1);
}

#[test]
fn buffered_frame_uses_exact_payload_and_preserves_unused_scratch() {
    let spi = MockSpi::new([ExpectedTransaction::frame(&PAYLOAD)]);
    let mut leds = Ws2812::new(spi);
    let mut scratch = [0x5a; 20];
    leds.write_colors_buffered(&[COLOR], &mut scratch).unwrap();
    assert_eq!(&scratch[..12], &PAYLOAD);
    assert_eq!(&scratch[12..], &[0x5a; 8]);
    leds.release().done(1);
}

#[test]
fn undersized_buffer_is_reported_before_mutation_or_io() {
    let mut leds = Ws2812::new(MockSpi::new([]));
    let mut scratch = [0x5a; 23];
    assert_eq!(
        leds.write_colors_buffered(&[COLOR, COLOR], &mut scratch),
        Err(Error::BufferTooSmall {
            required: 24,
            provided: 23,
        })
    );
    assert_eq!(scratch, [0x5a; 23]);
    leds.release().done(0);
}

#[test]
fn empty_inputs_do_not_touch_bus_or_scratch() {
    let mut leds = Ws2812::new(MockSpi::new([]));
    leds.write_colors(&[]).unwrap();
    let mut scratch = [0x5a; 12];
    leds.write_colors_buffered(&[], &mut scratch).unwrap();
    leds.write_colors_buffered(&[], &mut []).unwrap();
    assert_eq!(scratch, [0x5a; 12]);
    leds.release().done(0);
}

#[test]
fn frequency_validation_accepts_only_datasheet_timing_range() {
    for frequency in [3_000_000, 3_100_000, 3_200_000] {
        Ws2812::try_new(MockSpi::new([]), frequency)
            .unwrap()
            .release()
            .done(0);
    }
    for frequency in [0, 2_999_999, 3_200_001, u32::MAX] {
        assert!(matches!(
            Ws2812::try_new(MockSpi::new([]), frequency),
            Err(Error::InvalidSpiFrequency {
                actual_hz,
                minimum_hz: 3_000_000,
                maximum_hz: 3_200_000,
            }) if actual_hz == frequency
        ));
    }
}

#[test]
fn latch_sends_only_a_reset_interval() {
    let mut leds = Ws2812::new(MockSpi::new([ExpectedTransaction::latch()]));
    leds.latch().unwrap();
    leds.release().done(1);
}

#[test]
fn frame_failures_preserve_original_error_and_next_frame_starts_with_reset() {
    for fail_at in 0..3 {
        let mut failing = ExpectedTransaction::frame(&PAYLOAD);
        failing.fail_at = Some(fail_at);
        let mut leds = Ws2812::new(MockSpi::new([
            failing,
            ExpectedTransaction::frame(&PAYLOAD),
        ]));
        assert_eq!(
            leds.write_color(&COLOR),
            Err(Error::Interface(ErrorKind::ModeFault))
        );
        leds.write_colors(&[COLOR]).unwrap();
        leds.release().done(2);
    }
}

#[test]
fn buffered_write_and_latch_forward_transport_errors() {
    let mut failing_frame = ExpectedTransaction::frame(&PAYLOAD);
    failing_frame.fail_at = Some(1);
    let mut failing_latch = ExpectedTransaction::latch();
    failing_latch.fail_at = Some(0);
    let mut leds = Ws2812::new(MockSpi::new([failing_frame, failing_latch]));
    assert_eq!(
        leds.write_colors_buffered(&[COLOR], &mut [0; 12]),
        Err(Error::Interface(ErrorKind::ModeFault))
    );
    assert_eq!(leds.latch(), Err(Error::Interface(ErrorKind::ModeFault)));
    leds.release().done(2);
}

#[cfg(feature = "async")]
fn run_ready<F: core::future::Future>(future: F) -> F::Output {
    use core::task::{Context, Poll, Waker};
    let mut future = core::pin::pin!(future);
    match future
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("test mock unexpectedly suspended"),
    }
}

#[cfg(feature = "async")]
#[test]
fn async_methods_use_same_frame_layout_and_empty_input_semantics() {
    let mut payload = PAYLOAD.to_vec();
    payload.extend_from_slice(&[0xcc; 12]);
    let mut leds = Ws2812::new(MockSpi::new([
        ExpectedTransaction::frame(&PAYLOAD),
        ExpectedTransaction::frame(&payload),
        ExpectedTransaction::frame(&PAYLOAD),
        ExpectedTransaction::latch(),
    ]));
    run_ready(leds.write_color_async(&COLOR)).unwrap();
    run_ready(leds.write_colors_async(&[COLOR, RgbColor::new(255, 255, 255)])).unwrap();
    let mut scratch = [0x5a; 20];
    run_ready(leds.write_colors_buffered_async(&[COLOR], &mut scratch)).unwrap();
    assert_eq!(&scratch[12..], &[0x5a; 8]);
    run_ready(leds.write_colors_async(&[])).unwrap();
    run_ready(leds.write_colors_buffered_async(&[], &mut scratch)).unwrap();
    run_ready(leds.latch_async()).unwrap();
    leds.release().done(4);
}

#[cfg(feature = "async")]
#[test]
fn async_frame_failures_preserve_error_at_every_operation() {
    for fail_at in 0..3 {
        let mut failing = ExpectedTransaction::frame(&PAYLOAD);
        failing.fail_at = Some(fail_at);
        let mut leds = Ws2812::new(MockSpi::new([
            failing,
            ExpectedTransaction::frame(&PAYLOAD),
        ]));
        assert_eq!(
            run_ready(leds.write_colors_async(&[COLOR])),
            Err(Error::Interface(ErrorKind::ModeFault))
        );
        run_ready(leds.write_color_async(&COLOR)).unwrap();
        leds.release().done(2);
    }
}

#[cfg(feature = "async")]
#[test]
fn async_buffer_validation_precedes_mutation_and_bus_access() {
    let mut leds = Ws2812::new(MockSpi::new([]));
    let mut scratch = [0x5a; 11];
    assert_eq!(
        run_ready(leds.write_colors_buffered_async(&[COLOR], &mut scratch)),
        Err(Error::BufferTooSmall {
            required: 12,
            provided: 11
        })
    );
    assert_eq!(scratch, [0x5a; 11]);
    leds.release().done(0);
}
