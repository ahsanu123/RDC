use std::{cell::RefCell, rc::Rc};

use embedded_hal::delay::DelayNs;
use embedded_hal_mock::eh1::{
    delay::NoopDelay,
    serial::{Mock, Transaction},
};
use embedded_hal_nb::serial::{ErrorKind, ErrorType, Read, Write};
use tmc2209::{
    CommunicationSettings, DeviceError, EchoMode, MicrostepResolution, TMC2209, UartOperation,
    builder::Builder,
    register::{
        AutomaticPwmValues, ChopperConfiguration, DriverStatus, FactoryConfiguration,
        GeneralConfiguration, GlobalStatus, InputStatus, InterfaceCounter, MicrostepCounter,
        MicrostepCurrent, OtpConfiguration, PwmConfiguration, PwmScaling, ReadableRegister,
        StallDetectionResult, StepInterval,
    },
    u2, u3, u4, u5,
    validation::crc8,
};

fn settings(echo: bool) -> CommunicationSettings {
    CommunicationSettings::builder()
        .echo(if echo {
            EchoMode::Enabled
        } else {
            EchoMode::Disabled
        })
        .polling_attempts(3)
        .build()
}

fn request(address: u8, register: u8) -> [u8; 4] {
    let mut bytes = [0x05, address, register, 0];
    bytes[3] = crc8(&bytes[..3]);
    bytes
}

fn response(register: u8, value: u32) -> [u8; 8] {
    let [a, b, c, d] = value.to_be_bytes();
    let mut bytes = [0x05, 0xff, register, a, b, c, d, 0];
    bytes[7] = crc8(&bytes[..7]);
    bytes
}

fn send(expectations: &mut Vec<Transaction<u8>>, bytes: &[u8], echo: bool) {
    for &byte in bytes {
        expectations.push(Transaction::write(byte));
        if echo {
            expectations.push(Transaction::read(byte));
        }
    }
    expectations.push(Transaction::flush());
}

fn read_at(
    expectations: &mut Vec<Transaction<u8>>,
    address: u8,
    register: u8,
    value: u32,
    echo: bool,
) {
    send(expectations, &request(address, register), echo);
    expectations.push(Transaction::read_many(response(register, value)));
}

fn read(expectations: &mut Vec<Transaction<u8>>, register: u8, value: u32) {
    read_at(expectations, 0, register, value, false);
}

fn write_frame(expectations: &mut Vec<Transaction<u8>>, register: u8, value: u32, echo: bool) {
    let [a, b, c, d] = value.to_be_bytes();
    let mut bytes = [0x05, 0, register | 0x80, a, b, c, d, 0];
    bytes[7] = crc8(&bytes[..7]);
    send(expectations, &bytes, echo);
}

fn checked_write(expectations: &mut Vec<Transaction<u8>>, register: u8, value: u32, before: u8) {
    read(expectations, 0x02, u32::from(before));
    write_frame(expectations, register, value, false);
    read(expectations, 0x02, u32::from(before.wrapping_add(1)));
}

fn connected(uart: &mut Mock<u8>) -> TMC2209<&mut Mock<u8>, NoopDelay> {
    TMC2209::new(uart, NoopDelay, settings(false)).unwrap()
}

fn identity() -> Vec<Transaction<u8>> {
    let mut expectations = Vec::new();
    read(&mut expectations, 0x06, 0x2100_0000);
    expectations
}

#[test]
fn test_connection_all_addresses_echo_topologies_and_builder() {
    for address in 0..4 {
        for echo in [false, true] {
            let mut expectations = Vec::new();
            read_at(&mut expectations, address, 0x06, 0x2100_0040, echo);
            let mut uart = Mock::new(&expectations);
            let settings = CommunicationSettings {
                address: u2::new(address),
                ..settings(echo)
            };
            let device = Builder::builder()
                .uart(&mut uart)
                .delay(NoopDelay)
                .settings(settings)
                .build()
                .connect()
                .unwrap();
            device.release();
            uart.done();
        }
    }
}

#[test]
fn test_invalid_settings_rejected_before_uart_access() {
    for settings in [
        CommunicationSettings {
            baud_rate: 0,
            ..settings(false)
        },
        CommunicationSettings {
            baud_rate: 8_999,
            ..settings(false)
        },
        CommunicationSettings {
            baud_rate: 1_000_000,
            ..settings(false)
        },
        CommunicationSettings {
            clock_hz: 0,
            ..settings(false)
        },
        CommunicationSettings {
            polling_attempts: 0,
            ..settings(false)
        },
        CommunicationSettings {
            polling_delay_us: 0,
            ..settings(false)
        },
    ] {
        let mut uart: Mock<u8> = Mock::new(&[]);
        assert!(matches!(
            TMC2209::new(&mut uart, NoopDelay, settings),
            Err(DeviceError::InvalidCommunicationSettings)
        ));
        uart.done();
    }
}

#[test]
fn test_connection_rejects_wrong_silicon_without_writes() {
    let mut expectations = Vec::new();
    read(&mut expectations, 0x06, 0x2000_0000);
    let mut uart = Mock::new(&expectations);
    assert!(matches!(
        TMC2209::new(&mut uart, NoopDelay, settings(false)),
        Err(DeviceError::UnsupportedVersion { actual: 0x20 })
    ));
    uart.done();
}

fn read_typed<R: ReadableRegister>(device: &mut TMC2209<&mut Mock<u8>, NoopDelay>, expected: u32) {
    let actual: u32 = device.read_register::<R>().unwrap().into();
    assert_eq!(actual, expected);
}

#[test]
fn test_every_readable_register_uses_its_documented_address() {
    let addresses = [
        0x00, 0x01, 0x02, 0x05, 0x06, 0x07, 0x12, 0x41, 0x6a, 0x6b, 0x6c, 0x6f, 0x70, 0x71, 0x72,
    ];
    let mut expectations = identity();
    for address in addresses {
        read(&mut expectations, address, 0x1234_5678 + u32::from(address));
    }
    let mut uart = Mock::new(&expectations);
    let mut device = connected(&mut uart);
    read_typed::<GeneralConfiguration>(&mut device, 0x1234_5678);
    read_typed::<GlobalStatus>(&mut device, 0x1234_5679);
    read_typed::<InterfaceCounter>(&mut device, 0x1234_567a);
    read_typed::<OtpConfiguration>(&mut device, 0x1234_567d);
    read_typed::<InputStatus>(&mut device, 0x1234_567e);
    read_typed::<FactoryConfiguration>(&mut device, 0x1234_567f);
    read_typed::<StepInterval>(&mut device, 0x1234_568a);
    read_typed::<StallDetectionResult>(&mut device, 0x1234_56b9);
    read_typed::<MicrostepCounter>(&mut device, 0x1234_56e2);
    read_typed::<MicrostepCurrent>(&mut device, 0x1234_56e3);
    read_typed::<ChopperConfiguration>(&mut device, 0x1234_56e4);
    read_typed::<DriverStatus>(&mut device, 0x1234_56e7);
    read_typed::<PwmConfiguration>(&mut device, 0x1234_56e8);
    read_typed::<PwmScaling>(&mut device, 0x1234_56e9);
    read_typed::<AutomaticPwmValues>(&mut device, 0x1234_56ea);
    uart.done();
}

#[test]
fn test_current_write_byte_order_and_counter_rollover() {
    let mut expectations = identity();
    checked_write(&mut expectations, 0x10, 0x000a_1f03, 255);
    let mut uart = Mock::new(&expectations);
    connected(&mut uart)
        .set_currents(u5::new(3), u5::new(31), u4::new(10))
        .unwrap();
    uart.done();
}

#[test]
fn test_write_echo_is_consumed_per_byte_and_counter_verified() {
    let mut expectations = Vec::new();
    read_at(&mut expectations, 0, 0x06, 0x2100_0000, true);
    read_at(&mut expectations, 0, 0x02, 4, true);
    write_frame(&mut expectations, 0x22, 0x00ff_ffff, true);
    read_at(&mut expectations, 0, 0x02, 5, true);
    let mut uart = Mock::new(&expectations);
    let mut device = TMC2209::new(&mut uart, NoopDelay, settings(true)).unwrap();
    device.set_velocity(-1).unwrap();
    uart.done();
}

#[test]
fn test_unacknowledged_write_is_not_retried() {
    let mut expectations = identity();
    read(&mut expectations, 0x02, 7);
    write_frame(&mut expectations, 0x22, 0, false);
    read(&mut expectations, 0x02, 7);
    let mut uart = Mock::new(&expectations);
    assert_eq!(
        connected(&mut uart).set_velocity(0),
        Err(DeviceError::WriteNotAcknowledged {
            expected: 8,
            actual: 7
        })
    );
    uart.done();
}

#[test]
fn test_disable_preserves_unrelated_readable_chopper_fields() {
    let mut expectations = identity();
    read(&mut expectations, 0x6c, 0x1401_0053);
    checked_write(&mut expectations, 0x6c, 0x1401_0050, 1);
    let mut uart = Mock::new(&expectations);
    connected(&mut uart).disable().unwrap();
    uart.done();
}

#[test]
fn test_direction_preserves_unrelated_general_fields() {
    let mut expectations = identity();
    read(&mut expectations, 0x00, 0x41);
    checked_write(&mut expectations, 0x00, 0x49, 8);
    let mut uart = Mock::new(&expectations);
    connected(&mut uart).set_reversed_direction(true).unwrap();
    uart.done();
}

#[test]
fn test_enable_chopper_mode_and_standstill_mode_preserve_other_fields() {
    let mut expectations = identity();
    read(&mut expectations, 0x6c, 0x1401_0050);
    checked_write(&mut expectations, 0x6c, 0x1401_0051, 0);
    read(&mut expectations, 0x00, 0x41);
    checked_write(&mut expectations, 0x00, 0x45, 1);
    read(&mut expectations, 0x70, 0xc10d_0024);
    checked_write(&mut expectations, 0x70, 0xc11d_0024, 2);
    let mut uart = Mock::new(&expectations);
    let mut device = connected(&mut uart);
    device.enable(u4::new(1)).unwrap();
    device.set_spread_cycle(true).unwrap();
    device
        .set_standstill_mode(tmc2209::StandstillMode::Freewheeling)
        .unwrap();
    uart.done();
}

#[test]
fn test_configure_uart_preserves_configuration_without_enabling_motor() {
    let mut expectations = identity();
    read(&mut expectations, 0x00, 0x81);
    checked_write(&mut expectations, 0x00, 0xc1, 0);
    checked_write(&mut expectations, 0x03, 0x200, 1);
    let mut uart = Mock::new(&expectations);
    connected(&mut uart).configure_uart(u4::new(2)).unwrap();
    uart.done();
}

#[test]
fn test_microstep_selection_preserves_chopper_and_sets_register_control() {
    let mut expectations = identity();
    read(&mut expectations, 0x6c, 0x1001_0053);
    read(&mut expectations, 0x00, 0x01);
    checked_write(&mut expectations, 0x6c, 0x1401_0053, 10);
    checked_write(&mut expectations, 0x00, 0x81, 11);
    let mut uart = Mock::new(&expectations);
    connected(&mut uart)
        .set_microsteps(MicrostepResolution::Microsteps16)
        .unwrap();
    uart.done();
}

#[test]
fn test_later_microstep_write_failure_is_returned_without_rollback_or_retry() {
    let mut expectations = identity();
    read(&mut expectations, 0x6c, 0x1001_0053);
    read(&mut expectations, 0x00, 1);
    checked_write(&mut expectations, 0x6c, 0x1401_0053, 10);
    read(&mut expectations, 0x02, 11);
    expectations.push(Transaction::write_error(
        0x05,
        nb::Error::Other(ErrorKind::Overrun),
    ));
    let mut uart = Mock::new(&expectations);
    assert_eq!(
        connected(&mut uart).set_microsteps(MicrostepResolution::Microsteps16),
        Err(DeviceError::Interface(ErrorKind::Overrun))
    );
    uart.done();
}

#[test]
fn test_pin_controlled_microsteps_are_not_reported_as_register_selection() {
    let mut expectations = identity();
    read(&mut expectations, 0x00, 1);
    let mut uart = Mock::new(&expectations);
    assert_eq!(
        connected(&mut uart).read_microsteps(),
        Err(DeviceError::PinControlledMicrosteps)
    );
    uart.done();
}

#[test]
fn test_invalid_write_values_rejected_before_write_or_counter_access() {
    let mut uart = Mock::new(&identity());
    let mut device = connected(&mut uart);
    assert_eq!(
        device.write_register(GeneralConfiguration::from(1 << 10)),
        Err(DeviceError::InvalidWriteBits {
            register: 0,
            bits: 1 << 10
        })
    );
    assert_eq!(
        device.write_register(GeneralConfiguration::from(1 << 9)),
        Err(DeviceError::TestModeEnabled)
    );
    assert_eq!(
        device.write_register(ChopperConfiguration::from(9 << 24)),
        Err(DeviceError::InvalidMicrostepResolution { actual: 9 })
    );
    assert_eq!(
        device.write_register(ChopperConfiguration::from(1)),
        Err(DeviceError::InvalidOffTime)
    );
    assert_eq!(
        device.write_register(ChopperConfiguration::from((15 << 7) | (4 << 4))),
        Err(DeviceError::InvalidHysteresis)
    );
    assert_eq!(
        device.write_register(PwmConfiguration::from(1 << 22)),
        Err(DeviceError::InvalidWriteBits {
            register: 0x70,
            bits: 1 << 22
        })
    );
    assert_eq!(device.enable(u4::new(0)), Err(DeviceError::InvalidOffTime));
    for velocity in [i32::MIN, -8_388_608, 8_388_608, i32::MAX] {
        assert_eq!(
            device.set_velocity(velocity),
            Err(DeviceError::InvalidVelocity { value: velocity })
        );
    }
    uart.done();
}

#[test]
fn test_double_edge_filter_conflicts_are_rejected_in_both_write_directions() {
    let mut expectations = identity();
    read(&mut expectations, 0x6c, 1 << 29);
    read(&mut expectations, 0x00, 1 << 8);
    let mut uart = Mock::new(&expectations);
    let mut device = connected(&mut uart);
    assert_eq!(
        device.write_register(GeneralConfiguration::from(1 << 8)),
        Err(DeviceError::IncompatibleStepFiltering)
    );
    assert_eq!(
        device.write_register(ChopperConfiguration::from(1 << 29)),
        Err(DeviceError::IncompatibleStepFiltering)
    );
    uart.done();
}

#[test]
fn test_signed_velocity_boundaries_have_exact_24_bit_encoding() {
    for (velocity, raw) in [
        (-8_388_607, 0x0080_0001),
        (-1, 0x00ff_ffff),
        (0, 0),
        (8_388_607, 0x007f_ffff),
    ] {
        let mut expectations = identity();
        checked_write(&mut expectations, 0x22, raw, 0);
        let mut uart = Mock::new(&expectations);
        connected(&mut uart).set_velocity(velocity).unwrap();
        uart.done();
    }
}

#[test]
fn test_status_acknowledgement_writes_only_requested_flags_without_status_read() {
    let mut expectations = identity();
    checked_write(&mut expectations, 0x01, 2, 0);
    let mut uart = Mock::new(&expectations);
    connected(&mut uart)
        .clear_status_flags(false, true)
        .unwrap();
    uart.done();
}

#[test]
fn test_read_processing_uses_hardware_values_and_configured_clock() {
    let mut expectations = identity();
    read(&mut expectations, 0x00, 0x80);
    read(&mut expectations, 0x6c, 4 << 24);
    read(&mut expectations, 0x6b, (0x101 << 16) | 0xff);
    read(&mut expectations, 0x71, 0x1ff << 16);
    read(&mut expectations, 0x12, 1_200);
    read(&mut expectations, 0x00, 0);
    read(&mut expectations, 0x6c, 0);
    read(&mut expectations, 0x6f, 31 << 16);
    let mut uart = Mock::new(&expectations);
    let mut device = connected(&mut uart);
    assert_eq!(device.read_microsteps().unwrap(), 16);
    assert_eq!(device.read_phase_currents().unwrap(), (-255, 255));
    assert_eq!(device.read_pwm_correction().unwrap(), -1);
    assert_eq!(
        device.measured_256_microstep_frequency_hz().unwrap(),
        10_000.0
    );
    assert!((device.estimated_rms_current_amperes(0.11).unwrap() - 1.767_767).abs() < 0.000_01);
    uart.done();
}

#[test]
fn test_current_estimation_rejects_invalid_resistance_and_unsupported_scaling() {
    let mut expectations = identity();
    read(&mut expectations, 0, 1);
    read(&mut expectations, 0, 2);
    let mut uart = Mock::new(&expectations);
    let mut device = connected(&mut uart);
    for resistance in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        assert_eq!(
            device.estimated_rms_current_amperes(resistance),
            Err(DeviceError::InvalidSenseResistance)
        );
    }
    for _ in 0..2 {
        assert_eq!(
            device.estimated_rms_current_amperes(0.11),
            Err(DeviceError::UnsupportedCurrentEstimation)
        );
    }
    uart.done();
}

#[test]
fn test_echo_mismatch_and_invalid_crc_are_returned_not_scanned_past() {
    let mut uart = Mock::new(&[Transaction::write(0x05), Transaction::read(0x04)]);
    assert!(matches!(
        TMC2209::new(&mut uart, NoopDelay, settings(true)),
        Err(DeviceError::EchoMismatch {
            expected: 5,
            actual: 4
        })
    ));
    uart.done();

    let mut expectations = Vec::new();
    send(&mut expectations, &request(0, 6), false);
    let mut reply = response(6, 0x2100_0000);
    reply[7] ^= 1;
    expectations.push(Transaction::read_many(reply));
    let mut uart = Mock::new(&expectations);
    assert!(matches!(
        TMC2209::new(&mut uart, NoopDelay, settings(false)),
        Err(DeviceError::CrcMismatch { .. })
    ));
    uart.done();
}

/// Records delays used by the section 4 UART protocol and section 16 OTP sequence.
#[derive(Clone)]
struct RecordingDelay {
    samples: Rc<RefCell<Vec<u32>>>,
}

impl DelayNs for RecordingDelay {
    fn delay_ns(&mut self, ns: u32) {
        self.samples.borrow_mut().push(ns);
    }
}

#[test]
fn test_would_block_retries_are_bounded_for_transmit_receive_and_flush() {
    for operation in [
        UartOperation::Transmit,
        UartOperation::Receive,
        UartOperation::Flush,
    ] {
        let mut expectations = Vec::new();
        match operation {
            UartOperation::Transmit => {
                expectations
                    .extend((0..3).map(|_| Transaction::write_error(5, nb::Error::WouldBlock)));
            }
            UartOperation::Receive => {
                send(&mut expectations, &request(0, 6), false);
                expectations.extend((0..3).map(|_| Transaction::read_error(nb::Error::WouldBlock)));
            }
            UartOperation::Flush => {
                expectations.push(Transaction::write_many(request(0, 6)));
                expectations
                    .extend((0..3).map(|_| Transaction::flush_error(nb::Error::WouldBlock)));
            }
        }
        let samples = Rc::new(RefCell::new(Vec::new()));
        let delay = RecordingDelay {
            samples: samples.clone(),
        };
        let mut uart = Mock::new(&expectations);
        assert!(matches!(
            TMC2209::new(&mut uart, delay, settings(false)),
            Err(DeviceError::Timeout { operation: actual }) if actual == operation
        ));
        assert_eq!(&*samples.borrow(), &[139_000, 10_000, 10_000]);
        uart.done();
    }
}

#[test]
fn test_temporary_would_block_can_complete_without_restarting_frame() {
    let mut expectations = vec![Transaction::write_error(5, nb::Error::WouldBlock)];
    send(&mut expectations, &request(0, 6), false);
    expectations.push(Transaction::read_error(nb::Error::WouldBlock));
    expectations.push(Transaction::read_many(response(6, 0x2100_0000)));
    let mut uart = Mock::new(&expectations);
    TMC2209::new(&mut uart, NoopDelay, settings(false))
        .unwrap()
        .release();
    uart.done();
}

#[test]
fn test_write_failures_before_send_during_flush_and_after_acceptance_are_forwarded() {
    for stage in 0..3 {
        let mut expectations = identity();
        read(&mut expectations, 0x02, 1);
        if stage == 0 {
            expectations.push(Transaction::write_error(
                5,
                nb::Error::Other(ErrorKind::Parity),
            ));
        } else {
            let mut bytes = [5, 0, 0xa2, 0, 0, 0, 1, 0];
            bytes[7] = crc8(&bytes[..7]);
            expectations.push(Transaction::write_many(bytes));
            if stage == 1 {
                expectations.push(Transaction::flush_error(nb::Error::Other(
                    ErrorKind::Parity,
                )));
            } else {
                expectations.push(Transaction::flush());
                send(&mut expectations, &request(0, 2), false);
                expectations.push(Transaction::read_error(nb::Error::Other(ErrorKind::Parity)));
            }
        }
        let mut uart = Mock::new(&expectations);
        assert_eq!(
            connected(&mut uart).set_velocity(1),
            Err(DeviceError::Interface(ErrorKind::Parity))
        );
        uart.done();
    }
}

#[test]
fn test_recovery_returns_discarded_count_and_has_a_drain_limit() {
    let mut expectations = identity();
    expectations.extend([
        Transaction::flush(),
        Transaction::read(0x01),
        Transaction::read(0x02),
        Transaction::read_error(nb::Error::WouldBlock),
        Transaction::flush(),
        Transaction::read(0x01),
        Transaction::read(0x02),
        Transaction::read(0x03),
        Transaction::flush(),
        Transaction::read_error(nb::Error::Other(ErrorKind::Overrun)),
    ]);
    let mut uart = Mock::new(&expectations);
    let mut device = connected(&mut uart);
    assert_eq!(device.recover_uart(), Ok(2));
    assert_eq!(
        device.recover_uart(),
        Err(DeviceError::RecoveryLimitExceeded { discarded: 3 })
    );
    assert_eq!(
        device.recover_uart(),
        Err(DeviceError::Interface(ErrorKind::Overrun))
    );
    uart.done();
}

#[test]
fn test_otp_programming_is_explicit_waits_and_verifies_one_bit() {
    let mut expectations = identity();
    read(&mut expectations, 0x05, 0);
    read(&mut expectations, 0x02, 255);
    write_frame(&mut expectations, 0x04, 0xbd13, false);
    read(&mut expectations, 0x02, 0);
    read(&mut expectations, 0x04, 0);
    read(&mut expectations, 0x05, 0x800);
    let samples = Rc::new(RefCell::new(Vec::new()));
    let delay = RecordingDelay {
        samples: samples.clone(),
    };
    let mut uart = Mock::new(&expectations);
    let mut device = TMC2209::new(&mut uart, delay, settings(false)).unwrap();
    device.program_otp_bit(u2::new(1), u3::new(3)).unwrap();
    assert!(samples.borrow().contains(&10_000_000));
    uart.done();
}

#[test]
fn test_otp_invalid_location_existing_bit_and_failed_readback_are_errors() {
    let mut expectations = identity();
    read(&mut expectations, 0x05, 0x800);
    read(&mut expectations, 0x05, 0);
    read(&mut expectations, 0x02, 1);
    write_frame(&mut expectations, 0x04, 0xbd13, false);
    read(&mut expectations, 0x02, 2);
    read(&mut expectations, 0x04, 0);
    read(&mut expectations, 0x05, 0);
    let mut uart = Mock::new(&expectations);
    let mut device = connected(&mut uart);
    assert_eq!(
        device.program_otp_bit(u2::new(3), u3::new(0)),
        Err(DeviceError::InvalidOtpByte { actual: 3 })
    );
    assert_eq!(
        device.program_otp_bit(u2::new(1), u3::new(3)),
        Err(DeviceError::OtpBitAlreadyProgrammed)
    );
    assert_eq!(
        device.program_otp_bit(u2::new(1), u3::new(3)),
        Err(DeviceError::OtpVerificationFailed { byte: 1, bit: 3 })
    );
    uart.done();
}

#[derive(Debug, PartialEq, Eq)]
enum OwnedInterfaceError {
    Fault(Vec<u8>),
}

impl embedded_hal_nb::serial::Error for OwnedInterfaceError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

/// Injects owned interface errors at each section 4 UART operation.
struct FailingUart {
    operation: UartOperation,
    error: Option<OwnedInterfaceError>,
}

impl ErrorType for FailingUart {
    type Error = OwnedInterfaceError;
}

impl Read<u8> for FailingUart {
    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        assert_eq!(self.operation, UartOperation::Receive);
        Err(nb::Error::Other(self.error.take().unwrap()))
    }
}

impl Write<u8> for FailingUart {
    fn write(&mut self, _: u8) -> nb::Result<(), Self::Error> {
        if self.operation == UartOperation::Transmit {
            Err(nb::Error::Other(self.error.take().unwrap()))
        } else {
            Ok(())
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if self.operation == UartOperation::Flush {
            Err(nb::Error::Other(self.error.take().unwrap()))
        } else {
            Ok(())
        }
    }
}

#[test]
fn test_constructor_forwards_original_owned_uart_error_in_every_operation() {
    for operation in [
        UartOperation::Transmit,
        UartOperation::Receive,
        UartOperation::Flush,
    ] {
        let uart = FailingUart {
            operation,
            error: Some(OwnedInterfaceError::Fault(vec![0xde, 0xad])),
        };
        assert!(matches!(
            TMC2209::new(uart, NoopDelay, settings(false)),
            Err(DeviceError::Interface(OwnedInterfaceError::Fault(payload))) if payload == [0xde, 0xad]
        ));
    }
}
