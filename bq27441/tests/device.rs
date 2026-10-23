use bq27441::{
    BQ27441, BatteryConfiguration, DeviceError, Variant,
    register::{
        BlockData, ControlCommand, DataBlock, DataSubclass, HealthStatus, OperationConfiguration,
        Temperature,
    },
};
use embedded_hal::{delay::DelayNs, i2c::ErrorKind};
use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

#[derive(Debug, PartialEq, Eq)]
enum Delay {
    Nanoseconds(u32),
    Microseconds(u32),
    Milliseconds(u32),
}

#[derive(Default)]
struct RecordingDelay(Vec<Delay>);

impl DelayNs for RecordingDelay {
    fn delay_ns(&mut self, value: u32) {
        self.0.push(Delay::Nanoseconds(value));
    }
    fn delay_us(&mut self, value: u32) {
        self.0.push(Delay::Microseconds(value));
    }
    fn delay_ms(&mut self, value: u32) {
        self.0.push(Delay::Milliseconds(value));
    }
}

fn write(address: u8, value: u8) -> Transaction {
    Transaction::write(0x55, vec![address, value])
}

fn read_word(address: u8, value: u16) -> Transaction {
    Transaction::write_read(0x55, vec![address], value.to_le_bytes().to_vec())
}

fn command(value: u16) -> Vec<Transaction> {
    let [low, high] = value.to_le_bytes();
    vec![write(0, low), write(1, high)]
}

fn query(value: u16, response: u16) -> Vec<Transaction> {
    let mut operations = command(value);
    operations.push(read_word(0, response));
    operations
}

fn select(subclass: u8, page: u8) -> Vec<Transaction> {
    vec![write(0x61, 0), write(0x3e, subclass), write(0x3f, page)]
}

fn block_checksum(bytes: [u8; 32]) -> u8 {
    // Independent wider arithmetic oracle (not the driver's wrapping fold).
    255 - (bytes.iter().map(|&byte| u32::from(byte)).sum::<u32>() % 256) as u8
}

fn read_block(bytes: [u8; 32]) -> Vec<Transaction> {
    vec![
        Transaction::write_read(0x55, vec![0x40], bytes.to_vec()),
        Transaction::write_read(0x55, vec![0x60], vec![block_checksum(bytes)]),
    ]
}

fn write_block(bytes: [u8; 32]) -> Vec<Transaction> {
    let mut operations = select(82, 0);
    operations.extend(
        (0u8..32)
            .zip(bytes)
            .map(|(offset, byte)| write(0x40 + offset, byte)),
    );
    operations.push(write(0x60, block_checksum(bytes)));
    operations.extend(select(82, 0));
    operations.extend(read_block(bytes));
    operations
}

fn gauge(operations: &[Transaction]) -> BQ27441<Mock, RecordingDelay> {
    BQ27441::new(Mock::new(operations), RecordingDelay::default())
}

fn done(device: BQ27441<Mock, RecordingDelay>) -> Vec<Delay> {
    let (mut bus, delay) = device.release();
    bus.done();
    delay.0
}

#[test]
fn test_measurements_framing_byte_order_sign_temperature_and_health() {
    let operations = [
        read_word(0x04, 3821),
        read_word(0x10, 0xff38),
        read_word(0x12, 0xfffd),
        read_word(0x14, 0x8000),
        read_word(0x18, 0xfe70),
        read_word(0x02, 2731),
        read_word(0x1e, 2982),
        read_word(0x20, 0x0361),
        read_word(0x20, 0xff64),
        read_word(0x06, 0xc3bf),
    ];
    let mut device = gauge(&operations);
    assert_eq!(device.voltage_millivolts(), Ok(3821));
    assert_eq!(device.average_current_milliamps(), Ok(-200));
    assert_eq!(device.standby_current_milliamps(), Ok(-3));
    assert_eq!(device.maximum_load_current_milliamps(), Ok(-32768));
    assert_eq!(device.average_power_milliwatts(), Ok(-400));
    assert_eq!(device.temperature_centicelsius(), Ok(-5));
    assert_eq!(device.internal_temperature_centicelsius(), Ok(2505));
    let health = device.state_of_health().unwrap();
    assert_eq!(
        (health.percent(), health.status()),
        (97, HealthStatus::Learned)
    );
    assert_eq!(
        device.state_of_health().unwrap().status(),
        HealthStatus::Reserved(255)
    );
    let flags = device.flags().unwrap();
    assert!(flags.discharging() && flags.configuration_update() && flags.power_on_reset());
    assert!(flags.over_temperature() && flags.under_temperature() && flags.fully_charged());
    let delays = done(device);
    let mut expected = vec![Delay::Milliseconds(250)];
    for _ in operations {
        expected.extend([Delay::Microseconds(66), Delay::Milliseconds(500)]);
    }
    assert_eq!(delays, expected);
}

#[test]
fn test_all_capacity_and_charge_getters_use_correct_registers() {
    let addresses = [
        0x08, 0x0a, 0x0c, 0x0e, 0x1c, 0x28, 0x2a, 0x2c, 0x2e, 0x30, 0x3c,
    ];
    let operations: Vec<_> = addresses
        .into_iter()
        .map(|a| read_word(a, 0x1234))
        .collect();
    let mut device = gauge(&operations);
    assert_eq!(
        device.nominal_available_capacity_milliamp_hours(),
        Ok(0x1234)
    );
    assert_eq!(device.full_available_capacity_milliamp_hours(), Ok(0x1234));
    assert_eq!(device.remaining_capacity_milliamp_hours(), Ok(0x1234));
    assert_eq!(device.full_charge_capacity_milliamp_hours(), Ok(0x1234));
    assert_eq!(device.state_of_charge_percent(), Ok(0x1234));
    assert_eq!(
        device.remaining_capacity_unfiltered_milliamp_hours(),
        Ok(0x1234)
    );
    assert_eq!(
        device.remaining_capacity_filtered_milliamp_hours(),
        Ok(0x1234)
    );
    assert_eq!(
        device.full_charge_capacity_unfiltered_milliamp_hours(),
        Ok(0x1234)
    );
    assert_eq!(
        device.full_charge_capacity_filtered_milliamp_hours(),
        Ok(0x1234)
    );
    assert_eq!(device.state_of_charge_unfiltered_percent(), Ok(0x1234));
    assert_eq!(device.design_capacity_milliamp_hours(), Ok(0x1234));
    done(device);
}

#[test]
fn test_identify_both_variants_and_reject_inconsistent_identifiers() {
    for (kind, code, chemistry, expected) in [
        (0x0421, 0x48, 0x0128, Ok(Variant::G1A)),
        (0x0421, 0x58, 0x0312, Ok(Variant::G1B)),
        (
            0x0421,
            0x48,
            0x0312,
            Err(DeviceError::UnexpectedDevice {
                device_type: 0x0421,
                data_memory_code: 0x48,
                chemistry_identifier: 0x0312,
            }),
        ),
        (
            0x9999,
            0x48,
            0x0128,
            Err(DeviceError::UnexpectedDevice {
                device_type: 0x9999,
                data_memory_code: 0x48,
                chemistry_identifier: 0x0128,
            }),
        ),
    ] {
        let mut operations = query(1, kind);
        operations.extend(query(4, code));
        operations.extend(query(8, chemistry));
        let mut device = gauge(&operations);
        assert_eq!(device.identify(), expected);
        done(device);
    }
}

#[test]
fn test_control_query_pacing_and_invalid_command_directions() {
    let mut device = gauge(&query(2, 0x1234));
    assert_eq!(
        device
            .read_control(ControlCommand::FirmwareVersion)
            .unwrap()
            .value(),
        0x1234
    );
    assert_eq!(
        device.read_control(ControlCommand::Reset),
        Err(DeviceError::InvalidControlQuery)
    );
    assert_eq!(
        device.send_command(ControlCommand::Status),
        Err(DeviceError::InvalidControlAction)
    );
    assert_eq!(
        done(device),
        vec![
            Delay::Milliseconds(250),
            Delay::Microseconds(66),
            Delay::Microseconds(66),
            Delay::Milliseconds(2000),
            Delay::Microseconds(66),
            Delay::Milliseconds(500),
        ]
    );
}

#[test]
fn test_unseal_consecutive_keys_and_seal_verification() {
    let mut operations = command(0x8000);
    operations.extend(command(0x8000));
    operations.extend(query(0, 0x2000));
    operations.extend(query(0, 0));
    operations.extend(command(0x20));
    operations.extend(query(0, 0));
    operations.extend(query(0, 0x2000));
    let mut device = gauge(&operations);
    device.unseal().unwrap();
    device.seal().unwrap();
    done(device);
}

#[test]
fn test_seal_timeout_is_bounded() {
    let mut operations = command(0x20);
    for _ in 0..4 {
        operations.extend(query(0, 0));
    }
    let mut device = gauge(&operations);
    assert_eq!(
        device.seal(),
        Err(DeviceError::SealStateTimeout { expected: true })
    );
    done(device);
}

#[test]
fn test_configuration_entry_exit_polling_and_timeout() {
    for expected in [false, true] {
        let mut operations = query(0, 0);
        operations.extend(command(if expected { 0x13 } else { 0x42 }));
        operations.push(read_word(6, if expected { 0 } else { 0x10 }));
        operations.push(read_word(6, if expected { 0x10 } else { 0 }));
        let mut device = gauge(&operations);
        if expected {
            device.enter_configuration_update().unwrap();
        } else {
            device.exit_configuration_update().unwrap();
        }
        done(device);

        let mut operations = query(0, 0);
        operations.extend(command(if expected { 0x13 } else { 0x42 }));
        operations.extend((0..4).map(|_| read_word(6, if expected { 0 } else { 0x10 })));
        let mut device = gauge(&operations);
        let result = if expected {
            device.enter_configuration_update()
        } else {
            device.exit_configuration_update()
        };
        assert_eq!(
            result,
            Err(DeviceError::ConfigurationUpdateTimeout { expected })
        );
        done(device);
    }
}

#[test]
fn test_actions_check_sealing_and_shutdown_enable() {
    let mut device = gauge(&query(0, 0x2000));
    assert_eq!(
        device.send_command(ControlCommand::Reset),
        Err(DeviceError::Sealed)
    );
    done(device);
    let mut device = gauge(&query(0, 0));
    assert_eq!(
        device.send_command(ControlCommand::Shutdown),
        Err(DeviceError::ShutdownNotEnabled)
    );
    done(device);
    let mut operations = query(0, 0x8000);
    operations.extend(command(0x1c));
    let mut device = gauge(&operations);
    device.send_command(ControlCommand::Shutdown).unwrap();
    done(device);
    let mut operations = Vec::new();
    for value in [0x0c, 0x0d, 0x11, 0x12, 0x23] {
        operations.extend(command(value));
    }
    let mut device = gauge(&operations);
    for action in [
        ControlCommand::BatteryInsert,
        ControlCommand::BatteryRemove,
        ControlCommand::SetHibernate,
        ControlCommand::ClearHibernate,
        ControlCommand::PulseStateOfChargeInterrupt,
    ] {
        device.send_command(action).unwrap();
    }
    done(device);
}

#[test]
fn test_external_temperature_source_guard_and_bytewise_write() {
    let mut device = gauge(&[read_word(0x3a, 0x25f8)]);
    assert_eq!(
        device.write_temperature(Temperature::from(2982)),
        Err(DeviceError::ExternalTemperatureDisabled)
    );
    done(device);
    let mut device = gauge(&[read_word(0x3a, 0x25f9), write(2, 0xa6), write(3, 0x0b)]);
    device.write_temperature(Temperature::from(2982)).unwrap();
    assert_eq!(
        done(device),
        vec![
            Delay::Milliseconds(250),
            Delay::Microseconds(66),
            Delay::Microseconds(66),
            Delay::Microseconds(66),
            Delay::Milliseconds(2000),
        ]
    );
}

#[test]
fn test_operation_configuration_real_bit_positions_and_reserved_preservation() {
    let mut device = gauge(&[read_word(0x3a, 0x25f8)]);
    let mut config = device.operation_configuration().unwrap();
    assert!(
        config.battery_insert_enabled()
            && config.sleep_enabled()
            && config.remaining_capacity_tracks_full()
    );
    assert!(
        !config.external_temperature()
            && !config.battery_low_output()
            && !config.battery_insert_pull_up()
            && !config.output_active_high()
    );
    config.set_output_active_high(true);
    config.set_battery_insert_pull_up(true);
    config.set_battery_low_output(true);
    assert_eq!(u16::from(config), 0x3dfc);
    assert_eq!(OperationConfiguration::from(0xffff).reserved6().value(), 3);
    done(device);
}

#[test]
fn test_read_data_block_second_page_and_checksum_rejection() {
    let bytes = core::array::from_fn(|i| (i * 7 + 3) as u8);
    let mut operations = query(0, 0);
    operations.extend(select(80, 2));
    operations.extend(read_block(bytes));
    let mut device = gauge(&operations);
    assert_eq!(
        device.read_data_block(
            DataSubclass::ImpedanceTrackConfiguration,
            DataBlock::from(2)
        ),
        Ok(BlockData::from(bytes))
    );
    done(device);

    let mut operations = query(0, 0);
    operations.extend(select(82, 0));
    operations.push(Transaction::write_read(0x55, vec![0x40], bytes.to_vec()));
    operations.push(Transaction::write_read(
        0x55,
        vec![0x60],
        vec![block_checksum(bytes) ^ 1],
    ));
    let mut device = gauge(&operations);
    assert_eq!(
        device.read_data_block(DataSubclass::State, 0.into()),
        Err(DeviceError::ChecksumMismatch {
            expected: block_checksum(bytes),
            received: block_checksum(bytes) ^ 1,
        })
    );
    done(device);
}

#[test]
fn test_block_preconditions_reject_before_selection_or_write() {
    let mut device = gauge(&[]);
    assert_eq!(
        device.read_data_block(DataSubclass::State, 2.into()),
        Err(DeviceError::InvalidDataBlock)
    );
    assert_eq!(
        device.write_data_block(DataSubclass::SecurityCodes, 0.into(), [0; 32].into()),
        Err(DeviceError::ReadOnlyDataMemory)
    );
    done(device);
    let mut device = gauge(&query(0, 0x2000));
    assert_eq!(
        device.read_data_block(DataSubclass::State, 0.into()),
        Err(DeviceError::Sealed)
    );
    done(device);
    let mut operations = query(0, 0);
    operations.push(read_word(6, 0));
    let mut device = gauge(&operations);
    assert_eq!(
        device.write_data_block(DataSubclass::State, 0.into(), [0; 32].into()),
        Err(DeviceError::ConfigurationUpdateRequired)
    );
    done(device);
}

#[test]
fn test_write_data_block_commit_checksum_and_full_readback_verification() {
    let bytes = [0xa5; 32];
    let mut prefix = query(0, 0);
    prefix.push(read_word(6, 0x10));
    let mut operations = prefix.clone();
    operations.extend(write_block(bytes));
    let mut device = gauge(&operations);
    device
        .write_data_block(DataSubclass::State, 0.into(), bytes.into())
        .unwrap();
    let delays = done(device);
    assert_eq!(
        delays
            .iter()
            .filter(|d| **d == Delay::Milliseconds(10))
            .count(),
        1
    );

    let mut operations = prefix;
    let mut writes = write_block(bytes);
    writes.truncate(writes.len() - 2);
    operations.extend(writes);
    let mut changed = bytes;
    changed[31] ^= 0x80;
    operations.extend(read_block(changed));
    let mut device = gauge(&operations);
    assert_eq!(
        device.write_data_block(DataSubclass::State, 0.into(), bytes.into()),
        Err(DeviceError::BlockVerificationFailed)
    );
    done(device);
}

fn configuration() -> BatteryConfiguration {
    BatteryConfiguration {
        design_capacity_milliamp_hours: 1200,
        design_energy_milliwatt_hours: 4440,
        terminate_voltage_millivolts: 3200,
        taper_rate: 100,
    }
}

fn configuration_operations(sealed: bool, auto_reseal: bool) -> Vec<Transaction> {
    let mut operations = query(0, if sealed { 0x2000 } else { 0 });
    if sealed {
        operations.extend(command(0x8000));
        operations.extend(command(0x8000));
        operations.extend(query(0, 0));
    }
    operations.extend(query(0, 0));
    operations.extend(command(0x13));
    operations.push(read_word(6, 0x10));
    operations.extend(query(0, 0));
    operations.extend(select(82, 0));
    let mut original = core::array::from_fn(|i| (i * 3 + 1) as u8);
    if auto_reseal {
        original[2] |= 0x80;
    }
    operations.extend(read_block(original));
    let mut modified = original;
    // Independent expected bytes: capacity 1200, energy 4440, 3200 mV, rate 100.
    modified[10..14].copy_from_slice(&[0x04, 0xb0, 0x11, 0x58]);
    modified[16..18].copy_from_slice(&[0x0c, 0x80]);
    modified[27..29].copy_from_slice(&[0x00, 0x64]);
    operations.extend(write_block(modified));
    operations.extend(query(0, 0));
    operations.extend(command(0x42));
    operations.push(read_word(6, 0));
    if sealed {
        operations.extend(command(0x20));
        operations.extend(query(0, 0x2000));
    } else {
        operations.extend(query(0, if auto_reseal { 0x2000 } else { 0 }));
        if auto_reseal {
            operations.extend(command(0x8000));
            operations.extend(command(0x8000));
            operations.extend(query(0, 0));
        }
    }
    operations
}

#[test]
fn test_configure_battery_preserves_neighbors_big_endian_fields_and_sealing() {
    for (sealed, auto_reseal) in [(false, false), (false, true), (true, true)] {
        let mut device = gauge(&configuration_operations(sealed, auto_reseal));
        device.configure_battery(configuration()).unwrap();
        done(device);
    }
}

#[test]
fn test_configure_battery_validates_all_ranges_before_bus_io() {
    for (config, expected) in [
        (
            BatteryConfiguration {
                design_capacity_milliamp_hours: 199,
                ..configuration()
            },
            DeviceError::InvalidDesignCapacity,
        ),
        (
            BatteryConfiguration {
                design_capacity_milliamp_hours: 8001,
                ..configuration()
            },
            DeviceError::InvalidDesignCapacity,
        ),
        (
            BatteryConfiguration {
                design_energy_milliwatt_hours: 0,
                ..configuration()
            },
            DeviceError::InvalidDesignEnergy,
        ),
        (
            BatteryConfiguration {
                design_energy_milliwatt_hours: 32768,
                ..configuration()
            },
            DeviceError::InvalidDesignEnergy,
        ),
        (
            BatteryConfiguration {
                terminate_voltage_millivolts: 2499,
                ..configuration()
            },
            DeviceError::InvalidTerminateVoltage,
        ),
        (
            BatteryConfiguration {
                terminate_voltage_millivolts: 3701,
                ..configuration()
            },
            DeviceError::InvalidTerminateVoltage,
        ),
        (
            BatteryConfiguration {
                taper_rate: 0,
                ..configuration()
            },
            DeviceError::InvalidTaperRate,
        ),
        (
            BatteryConfiguration {
                taper_rate: 2001,
                ..configuration()
            },
            DeviceError::InvalidTaperRate,
        ),
    ] {
        let mut device = gauge(&[]);
        assert_eq!(device.configure_battery(config), Err(expected));
        done(device);
    }
}

#[test]
fn test_configure_battery_stops_and_forwards_error_at_every_transfer() {
    for (sealed, auto_reseal) in [(false, false), (false, true), (true, true)] {
        let operations = configuration_operations(sealed, auto_reseal);
        for index in 0..operations.len() {
            let mut failing = operations[..=index].to_vec();
            failing[index] = failing[index].clone().with_error(ErrorKind::Other);
            let mut device = gauge(&failing);
            assert_eq!(
                device.configure_battery(configuration()),
                Err(DeviceError::Interface(ErrorKind::Other)),
                "sealed={sealed}, transfer={index}"
            );
            done(device);
        }
    }
}

#[test]
fn test_configure_battery_accepts_exact_supported_range_boundaries() {
    for config in [
        BatteryConfiguration {
            design_capacity_milliamp_hours: 200,
            design_energy_milliwatt_hours: 1,
            terminate_voltage_millivolts: 2500,
            taper_rate: 1,
        },
        BatteryConfiguration {
            design_capacity_milliamp_hours: 8000,
            design_energy_milliwatt_hours: 32767,
            terminate_voltage_millivolts: 3700,
            taper_rate: 2000,
        },
    ] {
        let mut device = gauge(&[write(0, 0).with_error(ErrorKind::Other)]);
        assert_eq!(
            device.configure_battery(config),
            Err(DeviceError::Interface(ErrorKind::Other))
        );
        done(device);
    }
}

#[test]
fn test_interface_error_keeps_device_specific_payload() {
    use embedded_hal::i2c::{Error, ErrorType, I2c, Operation};

    #[derive(Debug, PartialEq, Eq)]
    struct PayloadError {
        code: u32,
    }
    impl Error for PayloadError {
        fn kind(&self) -> ErrorKind {
            ErrorKind::Other
        }
    }
    struct FailingBus;
    impl ErrorType for FailingBus {
        type Error = PayloadError;
    }
    impl I2c for FailingBus {
        fn transaction(
            &mut self,
            _address: u8,
            _operations: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            Err(PayloadError { code: 0xdeadbeef })
        }
    }
    let mut device = BQ27441::new(FailingBus, RecordingDelay::default());
    assert_eq!(
        device.voltage_millivolts(),
        Err(DeviceError::Interface(PayloadError { code: 0xdeadbeef }))
    );
}

#[test]
fn test_failed_standard_read_keeps_pacing_and_returns_original_error() {
    let mut device = gauge(&[read_word(4, 0).with_error(ErrorKind::ArbitrationLoss)]);
    assert_eq!(
        device.voltage_millivolts(),
        Err(DeviceError::Interface(ErrorKind::ArbitrationLoss))
    );
    assert_eq!(
        done(device),
        vec![
            Delay::Milliseconds(250),
            Delay::Microseconds(66),
            Delay::Milliseconds(500)
        ]
    );
}
