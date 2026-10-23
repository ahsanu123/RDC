use core::num::NonZeroU16;
use embedded_hal::{delay::DelayNs, i2c::ErrorKind};
use embedded_hal_mock::eh1::{
    i2c::{Mock, Transaction},
    spi::{Mock as SpiMock, Transaction as SpiTransaction},
};
use mmc5983ma::{
    Configuration, ConfigurationError, Error, Mmc5983ma,
    builder::Builder,
    comproc::{I2cInterface, SpiInterface},
    register::{Bandwidth, ContinuousFrequency},
};

#[derive(Default)]
struct Delay {
    nanoseconds: Vec<u32>,
}

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        self.nanoseconds.push(ns);
    }
}

fn read(address: u8, result: &[u8]) -> Transaction {
    Transaction::write_read(0x30, vec![address], result.to_vec())
}

fn write(address: u8, value: u8) -> Transaction {
    Transaction::write(0x30, vec![address, value])
}

fn startup() -> Vec<Transaction> {
    vec![
        read(0x2f, &[0x30]),
        write(0x0a, 0x80),
        read(0x08, &[0x10]),
        write(0x09, 0x08),
    ]
}

fn device(expectations: &[Transaction]) -> Mmc5983ma<I2cInterface<Mock>> {
    Builder::builder()
        .interface(I2cInterface::new(Mock::new(expectations)))
        .poll_limit(NonZeroU16::new(3).unwrap())
        .build()
        .build_device()
}

fn done(sensor: Mmc5983ma<I2cInterface<Mock>>) {
    sensor.release().release().done();
}

fn measurement(command: u8, output_address: u8, output: &[u8]) -> Vec<Transaction> {
    vec![
        write(0x08, command),
        write(0x09, command),
        read(0x08, &[0x10 | command]),
        read(output_address, output),
    ]
}

#[test]
fn initialize_resets_checks_otp_sets_and_observes_delays() {
    let mut sensor = device(&startup());
    let mut delay = Delay::default();
    assert_eq!(sensor.configuration(), Err(Error::NotInitialized));
    sensor.initialize(&mut delay).unwrap();
    assert_eq!(sensor.configuration(), Ok(Configuration::default()));
    assert_eq!(delay.nanoseconds, [10_000_000, 10_000_000, 1_000_000]);
    done(sensor);
}

#[test]
fn initialize_rejects_wrong_identity_and_missing_otp() {
    let mut sensor = device(&[read(0x2f, &[0x31])]);
    assert_eq!(
        sensor.initialize(&mut Delay::default()),
        Err(Error::UnexpectedProductId(0x31))
    );
    done(sensor);

    let mut sensor = device(&[read(0x2f, &[0x30]), write(0x0a, 0x80), read(0x08, &[0])]);
    assert_eq!(
        sensor.initialize(&mut Delay::default()),
        Err(Error::OtpNotReady)
    );
    assert_eq!(sensor.configuration(), Err(Error::NotInitialized));
    done(sensor);
}

#[test]
fn initialize_forwards_failure_at_each_bus_step() {
    for step in 0..startup().len() {
        let mut transactions = startup();
        transactions[step] = transactions[step].clone().with_error(ErrorKind::Other);
        let mut sensor = device(&transactions[..=step]);
        assert_eq!(
            sensor.initialize(&mut Delay::default()),
            Err(Error::Interface(ErrorKind::Other)),
            "startup step {step}"
        );
        assert_eq!(sensor.configuration(), Err(Error::NotInitialized));
        done(sensor);
    }
}

#[test]
fn configuration_preserves_settings_in_commands_and_waits_after_stop() {
    let mut transactions = startup();
    transactions.extend([
        write(0x0b, 0),
        write(0x09, 0x24),
        write(0x0a, 1),
        write(0x0c, 2),
        write(0x0b, 0),
        write(0x09, 0x2c),
        write(0x09, 0x34),
        write(0x08, 2),
        write(0x09, 0x26),
        read(0x08, &[0x12]),
        read(0x07, &[125]),
        write(0x08, 1),
        write(0x09, 0x25),
        read(0x08, &[0x11]),
        read(0x00, &[0x80, 0, 0x80, 0, 0x80, 0, 0]),
        write(0x09, 0x64),
        read(0x08, &[0x10]),
    ]);
    let mut sensor = device(&transactions);
    let mut delay = Delay::default();
    sensor.initialize(&mut delay).unwrap();
    let mut configuration = Configuration::default();
    configuration
        .control0
        .set_measurement_interrupt_enabled(true);
    configuration.control0.set_automatic_set_reset_enabled(true);
    configuration.control1.set_bandwidth(Bandwidth::Hertz200);
    configuration.control3.set_positive_self_test_enabled(true);
    sensor.configure(configuration, &mut delay).unwrap();
    assert_eq!(delay.nanoseconds.last(), Some(&10_000_000));
    sensor.set(&mut delay).unwrap();
    sensor.reset(&mut delay).unwrap();
    assert_eq!(sensor.measure_temperature(&mut delay), Ok(25.0));
    assert_eq!(
        sensor.measure_magnetic_field(&mut delay).unwrap().raw,
        [131_072; 3]
    );
    sensor.reload_otp(&mut delay).unwrap();
    assert_eq!(sensor.configuration(), Ok(configuration));
    done(sensor);
}

#[test]
fn configuration_write_failures_invalidate_state_without_rollback() {
    let writes = [
        write(0x0b, 0),
        write(0x09, 0),
        write(0x0a, 0),
        write(0x0c, 0),
        write(0x0b, 0),
    ];
    for step in 0..writes.len() {
        let mut transactions = startup();
        transactions.extend_from_slice(&writes[..step]);
        transactions.push(writes[step].clone().with_error(ErrorKind::Other));
        let mut sensor = device(&transactions);
        let mut delay = Delay::default();
        sensor.initialize(&mut delay).unwrap();
        assert_eq!(
            sensor.configure(Configuration::default(), &mut delay),
            Err(Error::Interface(ErrorKind::Other))
        );
        assert_eq!(sensor.configuration(), Err(Error::NotInitialized));
        assert_eq!(sensor.set(&mut delay), Err(Error::NotInitialized));
        done(sensor);
    }
}

#[test]
fn invalid_configuration_is_rejected_without_bus_io() {
    // Bytes follow the datasheet register layout and exercise hardware constraints.
    let cases = [
        ([0x80, 0, 0, 0], ConfigurationError::ReservedBits),
        ([0, 0x20, 0, 0], ConfigurationError::ReservedBits),
        ([0, 0, 0, 0x08], ConfigurationError::ReservedBits),
        ([3, 0, 0, 0], ConfigurationError::CommandBits),
        ([0, 0x80, 0, 0], ConfigurationError::CommandBits),
        ([0, 8, 0, 0], ConfigurationError::InvalidChannelInhibit),
        ([0, 0, 8, 0], ConfigurationError::InvalidContinuousFrequency),
        ([0, 0, 0x0e, 0], ConfigurationError::InsufficientBandwidth),
        ([0, 1, 0x0f, 0], ConfigurationError::InsufficientBandwidth),
        (
            [0, 0, 0x80, 0],
            ConfigurationError::PeriodicSetRequiresAutomaticAndContinuous,
        ),
        ([0, 0, 0, 0x40], ConfigurationError::ThreeWireSpiUnsupported),
        ([0, 0, 0, 6], ConfigurationError::OpposingSelfTestCurrents),
    ];
    let mut sensor = device(&startup());
    let mut delay = Delay::default();
    sensor.initialize(&mut delay).unwrap();
    for ([c0, c1, c2, c3], error) in cases {
        let configuration = Configuration {
            control0: c0.into(),
            control1: c1.into(),
            control2: c2.into(),
            control3: c3.into(),
        };
        assert_eq!(
            sensor.configure(configuration, &mut delay),
            Err(Error::InvalidConfiguration(error))
        );
        assert_eq!(sensor.configuration(), Ok(Configuration::default()));
    }
    done(sensor);
}

#[test]
fn continuous_frequency_bandwidth_constraints_match_datasheet() {
    for (frequency, accepted_bandwidths) in [
        (ContinuousFrequency::Hertz1, [true, true, true, true]),
        (ContinuousFrequency::Hertz10, [true, true, true, true]),
        (ContinuousFrequency::Hertz20, [true, true, true, true]),
        (ContinuousFrequency::Hertz50, [true, true, true, true]),
        (ContinuousFrequency::Hertz100, [false, true, true, true]),
        (ContinuousFrequency::Hertz200, [false, true, false, false]),
        (ContinuousFrequency::Hertz1000, [false, false, false, true]),
    ] {
        for (bandwidth, accepted) in [
            Bandwidth::Hertz100,
            Bandwidth::Hertz200,
            Bandwidth::Hertz400,
            Bandwidth::Hertz800,
        ]
        .into_iter()
        .zip(accepted_bandwidths)
        {
            let mut configuration = Configuration::default();
            configuration.control0.set_automatic_set_reset_enabled(true);
            configuration.control1.set_bandwidth(bandwidth);
            configuration.control2.set_frequency(frequency);
            configuration
                .control2
                .set_continuous_measurement_enabled(true);
            configuration.control2.set_periodic_set_enabled(true);
            assert_eq!(configuration.validate().is_ok(), accepted);
        }
    }
}

#[test]
fn single_shot_polls_then_reads_one_seven_byte_burst() {
    let mut transactions = startup();
    transactions.extend([
        write(0x08, 1),
        write(0x09, 1),
        read(0x08, &[0x10]),
        read(0x08, &[0x11]),
        read(0x00, &[0x80, 0, 0x90, 0, 0x70, 0, 0x6c]),
    ]);
    let mut sensor = device(&transactions);
    let mut delay = Delay::default();
    sensor.initialize(&mut delay).unwrap();
    let result = sensor.measure_magnetic_field(&mut delay).unwrap();
    assert_eq!(result.raw, [131_073, 147_458, 114_691]);
    assert_eq!(&delay.nanoseconds[3..], &[1_000_000, 1_000_000]);
    done(sensor);
}

#[test]
fn measurement_timeout_is_bounded_and_requires_reinitialization() {
    let mut transactions = startup();
    transactions.extend([
        write(0x08, 1),
        write(0x09, 1),
        read(0x08, &[0x10]),
        read(0x08, &[0x10]),
        read(0x08, &[0x10]),
    ]);
    transactions.extend(startup());
    let mut sensor = device(&transactions);
    let mut delay = Delay::default();
    sensor.initialize(&mut delay).unwrap();
    assert_eq!(
        sensor.measure_magnetic_field(&mut delay),
        Err(Error::MeasurementTimeout)
    );
    assert_eq!(
        sensor.measure_temperature(&mut delay),
        Err(Error::NotInitialized)
    );
    sensor.initialize(&mut delay).unwrap();
    assert_eq!(sensor.configuration(), Ok(Configuration::default()));
    done(sensor);
}

#[test]
fn otp_reload_checks_otp_completion_instead_of_measurement_completion() {
    let mut transactions = startup();
    transactions.extend([write(0x09, 0x40), read(0x08, &[3]), read(0x08, &[0x10])]);
    let mut sensor = device(&transactions);
    let mut delay = Delay::default();
    sensor.initialize(&mut delay).unwrap();
    sensor.reload_otp(&mut delay).unwrap();
    assert_eq!(&delay.nanoseconds[3..], &[10_000_000, 1_000_000, 1_000_000]);
    done(sensor);
}

#[test]
fn acquisition_forwards_each_failure_and_invalidates_uncertain_state() {
    for (command, address, output) in [
        (1, 0, vec![0x80, 0, 0x80, 0, 0x80, 0, 0]),
        (2, 7, vec![125]),
    ] {
        let acquisition = measurement(command, address, &output);
        for step in 0..acquisition.len() {
            let mut transactions = startup();
            transactions.extend_from_slice(&acquisition[..step]);
            transactions.push(acquisition[step].clone().with_error(ErrorKind::Other));
            let mut sensor = device(&transactions);
            let mut delay = Delay::default();
            sensor.initialize(&mut delay).unwrap();
            let error = if command == 1 {
                sensor.measure_magnetic_field(&mut delay).unwrap_err()
            } else {
                sensor.measure_temperature(&mut delay).unwrap_err()
            };
            assert_eq!(error, Error::Interface(ErrorKind::Other));
            assert_eq!(sensor.configuration(), Err(Error::NotInitialized));
            done(sensor);
        }
    }
}

#[test]
fn read_methods_require_done_bits_and_do_not_acknowledge_interrupts() {
    let mut transactions = startup();
    transactions.extend([
        read(0x08, &[0x10]),
        read(0x08, &[0x10]),
        read(0x08, &[0x11]),
        read(0x00, &[0x80, 0, 0x80, 0, 0x80, 0, 0]),
        read(0x08, &[0x12]),
        read(0x07, &[125]),
        write(0x08, 1),
        write(0x08, 2),
        write(0x08, 3),
    ]);
    let mut sensor = device(&transactions);
    sensor.initialize(&mut Delay::default()).unwrap();
    assert_eq!(
        sensor.read_magnetic_field(),
        Err(Error::MeasurementNotReady)
    );
    assert_eq!(sensor.read_temperature(), Err(Error::MeasurementNotReady));
    assert_eq!(sensor.read_magnetic_field().unwrap().raw, [131_072; 3]);
    assert_eq!(sensor.read_temperature(), Ok(25.0));
    sensor.clear_interrupts(true, false).unwrap();
    sensor.clear_interrupts(false, true).unwrap();
    sensor.clear_interrupts(true, true).unwrap();
    done(sensor);
}

fn compensation() -> Vec<Transaction> {
    let mut transactions = vec![write(0x09, 8)];
    transactions.extend(measurement(1, 0, &[0x90, 0, 0x80, 0, 0x70, 0, 0]));
    transactions.push(write(0x09, 0x10));
    transactions.extend(measurement(1, 0, &[0x70, 0, 0x80, 0, 0x90, 0, 0]));
    transactions.push(write(0x09, 8));
    transactions
}

#[test]
fn compensation_measures_opposite_polarities_and_restores_set() {
    let mut transactions = startup();
    transactions.extend(compensation());
    let mut sensor = device(&transactions);
    let mut delay = Delay::default();
    sensor.initialize(&mut delay).unwrap();
    let result = sensor.measure_offset_compensated(&mut delay).unwrap();
    assert_eq!([result.x, result.y, result.z], [1.0, 0.0, -1.0]);
    assert_eq!(
        &delay.nanoseconds[3..],
        &[1_000_000, 1_000_000, 1_000_000, 1_000_000, 1_000_000]
    );
    done(sensor);
}

#[test]
fn compensation_forwards_each_failure_without_cleanup_io() {
    let acquisition = compensation();
    for step in 0..acquisition.len() {
        let mut transactions = startup();
        transactions.extend_from_slice(&acquisition[..step]);
        transactions.push(acquisition[step].clone().with_error(ErrorKind::Other));
        let mut sensor = device(&transactions);
        let mut delay = Delay::default();
        sensor.initialize(&mut delay).unwrap();
        assert_eq!(
            sensor.measure_offset_compensated(&mut delay),
            Err(Error::Interface(ErrorKind::Other)),
            "compensation step {step}"
        );
        assert_eq!(sensor.configuration(), Err(Error::NotInitialized));
        done(sensor);
    }
}

#[test]
fn continuous_mode_allows_reads_but_rejects_single_shot_commands() {
    let mut transactions = startup();
    transactions.extend([
        write(0x0b, 0),
        write(0x09, 0),
        write(0x0a, 3),
        write(0x0c, 0),
        write(0x0b, 0x0f),
        read(0x08, &[0x11]),
        read(0x00, &[0x80, 0, 0x80, 0, 0x80, 0, 0]),
        write(0x0b, 7),
        write(0x09, 0),
        write(0x0a, 0),
        write(0x0c, 0),
        write(0x0b, 0),
    ]);
    let mut sensor = device(&transactions);
    let mut delay = Delay::default();
    sensor.initialize(&mut delay).unwrap();
    let mut configuration = Configuration::default();
    configuration.control1.set_bandwidth(Bandwidth::Hertz800);
    configuration
        .control2
        .set_frequency(ContinuousFrequency::Hertz1000);
    configuration
        .control2
        .set_continuous_measurement_enabled(true);
    sensor.configure(configuration, &mut delay).unwrap();
    assert_eq!(
        sensor.measure_magnetic_field(&mut delay),
        Err(Error::ContinuousModeActive)
    );
    assert_eq!(
        sensor.measure_temperature(&mut delay),
        Err(Error::ContinuousModeActive)
    );
    assert_eq!(sensor.set(&mut delay), Err(Error::ContinuousModeActive));
    assert_eq!(sensor.reset(&mut delay), Err(Error::ContinuousModeActive));
    assert_eq!(
        sensor.reload_otp(&mut delay),
        Err(Error::ContinuousModeActive)
    );
    assert_eq!(sensor.read_magnetic_field().unwrap().raw, [131_072; 3]);
    sensor
        .configure(Configuration::default(), &mut delay)
        .unwrap();
    done(sensor);
}

#[test]
fn disabled_channels_and_automatic_set_reset_reject_incompatible_acquisition() {
    let mut transactions = startup();
    transactions.extend([
        write(0x0b, 0),
        write(0x09, 0),
        write(0x0a, 0x18),
        write(0x0c, 0),
        write(0x0b, 0),
        write(0x0b, 0),
        write(0x09, 0x20),
        write(0x0a, 0),
        write(0x0c, 0),
        write(0x0b, 0),
    ]);
    let mut sensor = device(&transactions);
    let mut delay = Delay::default();
    sensor.initialize(&mut delay).unwrap();
    let mut configuration = Configuration {
        control1: 0x18.into(),
        ..Configuration::default()
    };
    sensor.configure(configuration, &mut delay).unwrap();
    assert_eq!(
        sensor.measure_magnetic_field(&mut delay),
        Err(Error::ChannelsDisabled)
    );
    assert_eq!(sensor.read_magnetic_field(), Err(Error::ChannelsDisabled));
    assert_eq!(
        sensor.measure_offset_compensated(&mut delay),
        Err(Error::ChannelsDisabled)
    );
    configuration = Configuration::default();
    configuration.control0.set_automatic_set_reset_enabled(true);
    sensor.configure(configuration, &mut delay).unwrap();
    assert_eq!(
        sensor.measure_offset_compensated(&mut delay),
        Err(Error::AutomaticSetResetActive)
    );
    done(sensor);
}

#[test]
fn spi_initialization_and_burst_read_use_single_cs_transactions() {
    let mut expectations = Vec::new();
    for (address, result) in [(0x2f, 0x30), (0x08, 0x10)] {
        expectations.extend([
            SpiTransaction::transaction_start(),
            SpiTransaction::write_vec(vec![0x80 | address]),
            SpiTransaction::read(result),
            SpiTransaction::transaction_end(),
        ]);
        if address == 0x2f {
            expectations.extend([
                SpiTransaction::transaction_start(),
                SpiTransaction::write_vec(vec![0x0a, 0x80]),
                SpiTransaction::transaction_end(),
            ]);
        }
    }
    expectations.extend([
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x09, 8]),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x88]),
        SpiTransaction::read(0x11),
        SpiTransaction::transaction_end(),
        SpiTransaction::transaction_start(),
        SpiTransaction::write_vec(vec![0x80]),
        SpiTransaction::read_vec(vec![0x80, 0, 0x80, 0, 0x80, 0, 0x6c]),
        SpiTransaction::transaction_end(),
    ]);
    let mut sensor = Builder::builder()
        .interface(SpiInterface::new(SpiMock::new(&expectations)))
        .build()
        .build_device();
    sensor.initialize(&mut Delay::default()).unwrap();
    assert_eq!(
        sensor.read_magnetic_field().unwrap().raw,
        [131_073, 131_074, 131_075]
    );
    sensor.release().release().done();
}

#[test]
fn spi_forwards_transport_errors() {
    struct FailingSpi;
    impl embedded_hal::spi::ErrorType for FailingSpi {
        type Error = embedded_hal::spi::ErrorKind;
    }
    impl embedded_hal::spi::SpiDevice for FailingSpi {
        fn transaction(
            &mut self,
            _: &mut [embedded_hal::spi::Operation<'_, u8>],
        ) -> Result<(), Self::Error> {
            Err(embedded_hal::spi::ErrorKind::ModeFault)
        }
    }
    let mut sensor = Builder::builder()
        .interface(SpiInterface::new(FailingSpi))
        .build()
        .build_device();
    assert_eq!(
        sensor.read_product_id(),
        Err(Error::Interface(embedded_hal::spi::ErrorKind::ModeFault))
    );
}
