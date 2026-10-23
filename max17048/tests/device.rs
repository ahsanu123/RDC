use embedded_hal::i2c::{ErrorKind, NoAcknowledgeSource};
use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
use max17048::{
    Error, MAX17048,
    register::{
        CellVoltage, ChargeRate, Command, Configuration, HibernateThreshold, Mode, ModelTableWord,
        ResetVoltage, StateOfCharge, Status, Version, VoltageAlert,
    },
};

fn read(address: u8, word: u16) -> Transaction {
    Transaction::write_read(0x36, vec![address], word.to_be_bytes().to_vec())
}

fn write(address: u8, word: u16) -> Transaction {
    let [high, low] = word.to_be_bytes();
    Transaction::write(0x36, vec![address, high, low])
}

fn close(actual: f32, expected: f32) {
    assert!((actual - expected).abs() < 0.001, "{actual} != {expected}");
}

#[test]
fn ownership_performs_no_bus_traffic() {
    MAX17048::new(Mock::new(&[])).destroy().done();
}

#[test]
fn reads_all_typed_registers_as_big_endian_words() {
    let expectations: Vec<_> = [
        0x02, 0x04, 0x06, 0x08, 0x0a, 0x0c, 0x14, 0x16, 0x18, 0x1a, 0xfe,
    ]
    .into_iter()
    .map(|address| read(address, 0xa55a))
    .collect();
    let mut device = MAX17048::new(Mock::new(&expectations));
    macro_rules! check {
        ($($register:ty),*) => {
            $(assert_eq!(u16::from(device.read_register::<$register>().unwrap()), 0xa55a);)*
        };
    }
    check!(
        CellVoltage,
        StateOfCharge,
        Mode,
        Version,
        HibernateThreshold,
        Configuration,
        VoltageAlert,
        ChargeRate,
        ResetVoltage,
        Status,
        Command
    );
    device.destroy().done();
}

#[test]
fn measurements_use_correct_register_and_scaling() {
    let expectations = [
        read(0x02, 48000),
        read(0x04, 0x3280),
        read(0x16, 0xff9c),
        read(0x08, 0x0012),
        read(0x18, 0x96a5),
        read(0x0a, 0x8030),
        read(0x0a, 0x8030),
        read(0x0c, 0x971c),
        read(0x14, 0x96d2),
        read(0x18, 0x96a5),
    ];
    let mut device = MAX17048::new(Mock::new(&expectations));
    close(device.voltage().unwrap(), 3.75);
    close(device.soc().unwrap(), 50.5);
    close(device.charge_rate().unwrap(), -20.8);
    assert_eq!(device.version().unwrap(), 0x0012);
    assert_eq!(device.chip_id().unwrap(), 0xa5);
    close(device.hibernate_threshold().unwrap(), 26.624);
    close(device.activity_threshold().unwrap(), 0.06);
    assert_eq!(device.empty_alert_threshold().unwrap(), 4);
    let (minimum, maximum) = device.voltage_alert().unwrap();
    close(minimum, 3.0);
    close(maximum, 4.2);
    close(device.reset_voltage().unwrap(), 3.0);
    device.destroy().done();
}

#[test]
fn quick_start_preserves_sleep_and_reserved_bits() {
    let mut device = MAX17048::new(Mock::new(&[read(0x06, 0xb123), write(0x06, 0xf123)]));
    device.quick_start().unwrap();
    device.destroy().done();
}

#[test]
fn sleep_permission_never_replays_quick_start() {
    let expectations = [
        read(0x06, 0xd123),
        write(0x06, 0xb123),
        read(0x06, 0xf123),
        write(0x06, 0x9123),
    ];
    let mut device = MAX17048::new(Mock::new(&expectations));
    device.enable_sleep(true).unwrap();
    device.enable_sleep(false).unwrap();
    device.destroy().done();
}

#[test]
fn software_sleep_enables_permission_then_wakes_without_reconfiguring_mode() {
    let expectations = [
        read(0x06, 0x0000),
        write(0x06, 0x2000),
        read(0x0c, 0x971c),
        write(0x0c, 0x97bc),
        read(0x0c, 0x97bc),
        write(0x0c, 0x973c),
    ];
    let mut device = MAX17048::new(Mock::new(&expectations));
    device.sleep(true).unwrap();
    device.sleep(false).unwrap();
    device.destroy().done();
}

#[test]
fn reads_power_and_alert_flags_without_clearing() {
    let mut device = MAX17048::new(Mock::new(&[
        read(0x06, 0x1000),
        read(0x06, 0),
        read(0x0c, 0x973c),
        read(0x0c, 0x971c),
        read(0x0c, 0x971c),
        read(0x1a, 0x7f34),
        read(0x0a, 0x8030),
    ]));
    assert!(device.is_hibernating().unwrap());
    assert!(!device.is_hibernating().unwrap());
    assert!(device.is_active_alert().unwrap());
    assert!(!device.is_active_alert().unwrap());
    assert_eq!(u16::from(device.get_config().unwrap()), 0x971c);
    assert_eq!(u16::from(device.get_status().unwrap()), 0x7f34);
    assert_eq!(u16::from(device.get_hibernate_threshold().unwrap()), 0x8030);
    device.destroy().done();
}

#[test]
fn hibernate_setters_preserve_the_other_threshold() {
    let expectations = [
        read(0x0a, 0x8030),
        write(0x0a, 0x1230),
        read(0x0a, 0x1230),
        write(0x0a, 0x1234),
        read(0x0a, 0x1234),
        write(0x0a, 0xffff),
        read(0x0a, 0xffff),
        write(0x0a, 0x0000),
    ];
    let mut device = MAX17048::new(Mock::new(&expectations));
    device.set_hibernate_entry_threshold(0x12).unwrap();
    device.set_activity_threshold(0x34).unwrap();
    device.set_hibernate_threshold(0xffff.into()).unwrap();
    device.set_hibernate_threshold(0.into()).unwrap();
    device.destroy().done();
}

#[test]
fn configuration_updates_preserve_alert_latch_and_other_fields() {
    let expectations = [
        read(0x0c, 0x97dc),
        write(0x0c, 0x97f6),
        read(0x0c, 0x97d6),
        write(0x0c, 0x55f6),
        read(0x0c, 0x55f6),
        write(0x0c, 0x55b6),
        read(0x0c, 0x55b6),
        write(0x0c, 0x55f6),
        read(0x0c, 0x55f6),
        write(0x0c, 0x1234),
        read(0x0c, 0x1234),
        write(0x0c, 0x1214),
    ];
    let mut device = MAX17048::new(Mock::new(&expectations));
    device.set_empty_alert_threshold(10).unwrap();
    device.set_compensation(0x55).unwrap();
    device.enable_state_of_charge_change_alert(false).unwrap();
    device.enable_state_of_charge_change_alert(true).unwrap();
    device.set_config(0x1214.into()).unwrap();
    device.clear_alert().unwrap();
    device.destroy().done();
}

#[test]
fn empty_alert_threshold_accepts_both_endpoints() {
    let mut device = MAX17048::new(Mock::new(&[
        read(0x0c, 0x971c),
        write(0x0c, 0x973f),
        read(0x0c, 0x973f),
        write(0x0c, 0x9720),
    ]));
    device.set_empty_alert_threshold(1).unwrap();
    device.set_empty_alert_threshold(32).unwrap();
    device.destroy().done();
}

#[test]
fn status_updates_preserve_reserved_bits_and_unselected_events() {
    let expectations = [
        read(0x1a, 0x8134),
        write(0x1a, 0xff34),
        read(0x1a, 0xc134),
        write(0x1a, 0xfd34),
        read(0x1a, 0xfd34),
        write(0x1a, 0xbf34),
    ];
    let mut device = MAX17048::new(Mock::new(&expectations));
    device.set_status(0x4000.into()).unwrap();
    // Only VH is selected; reserved bits and EnVR in the input are ignored.
    device.clear_status_flags(0xc2ff.into()).unwrap();
    device.set_status(0.into()).unwrap();
    device.destroy().done();
}

#[test]
fn selected_status_flags_are_cleared_independently() {
    for flag in [0x0100, 0x0200, 0x0400, 0x0800, 0x1000, 0x2000, 0x3f00, 0] {
        let mut device = MAX17048::new(Mock::new(&[
            read(0x1a, 0xc15a),
            write(0x1a, 0xff5a & !flag),
        ]));
        device.clear_status_flags(flag.into()).unwrap();
        device.destroy().done();
    }
}

#[test]
fn voltage_alert_limits_are_ordered_and_big_endian() {
    for (minimum, maximum, word) in [(3000, 4200, 0x96d2), (0, 5100, 0x00ff), (0, 0, 0)] {
        let mut device = MAX17048::new(Mock::new(&[read(0x14, 0x00ff), write(0x14, word)]));
        device.set_voltage_alert(minimum, maximum).unwrap();
        device.destroy().done();
    }
}

#[test]
fn reset_voltage_preserves_id_and_comparator() {
    let mut device = MAX17048::new(Mock::new(&[
        read(0x18, 0x97a5),
        write(0x18, 0x9ba5),
        read(0x18, 0x9ba5),
        write(0x18, 0x9aa5),
        read(0x18, 0x9aa5),
        write(0x18, 0x9ba5),
        read(0x18, 0x9ba5),
        write(0x18, 0x01a5),
        read(0x18, 0x01a5),
        write(0x18, 0xffa5),
    ]));
    device.set_reset_voltage(3080).unwrap();
    device.enable_reset_comparator(true).unwrap();
    device.enable_reset_comparator(false).unwrap();
    device.set_reset_voltage(0).unwrap();
    device.set_reset_voltage(5080).unwrap();
    device.destroy().done();
}

#[test]
fn invalid_inputs_fail_without_bus_traffic() {
    let mut device = MAX17048::new(Mock::new(&[]));
    for percent in [0, 33, 255] {
        assert_eq!(
            device.set_empty_alert_threshold(percent),
            Err(Error::InvalidInputData)
        );
    }
    for (minimum, maximum) in [
        (4200, 3000),
        (0, 5120),
        (3001, 4200),
        (3000, 4201),
        (u16::MAX, u16::MAX),
    ] {
        assert_eq!(
            device.set_voltage_alert(minimum, maximum),
            Err(Error::InvalidInputData)
        );
    }
    for voltage in [1, 3001, 5081, 5120, u16::MAX] {
        assert_eq!(
            device.set_reset_voltage(voltage),
            Err(Error::InvalidInputData)
        );
    }
    device.destroy().done();
}

#[test]
fn read_errors_are_forwarded() {
    let mut device = MAX17048::new(Mock::new(&[
        read(0x02, 0).with_error(ErrorKind::ArbitrationLoss)
    ]));
    assert_eq!(
        device.voltage(),
        Err(Error::I2C(ErrorKind::ArbitrationLoss))
    );
    device.destroy().done();
}

#[test]
fn failed_read_does_not_attempt_dependent_write() {
    let mut device = MAX17048::new(Mock::new(&[read(0x0c, 0).with_error(ErrorKind::Bus)]));
    assert_eq!(
        device.set_compensation(0x55),
        Err(Error::I2C(ErrorKind::Bus))
    );
    device.destroy().done();
}

#[test]
fn write_errors_are_forwarded_without_retry() {
    let mut device = MAX17048::new(Mock::new(&[
        read(0x06, 0),
        write(0x06, 0x4000).with_error(ErrorKind::Bus),
    ]));
    assert_eq!(device.quick_start(), Err(Error::I2C(ErrorKind::Bus)));
    device.destroy().done();
}

#[test]
fn reset_command_nack_is_not_hidden() {
    let error = ErrorKind::NoAcknowledge(NoAcknowledgeSource::Data);
    let mut device = MAX17048::new(Mock::new(&[write(0xfe, 0x5400).with_error(error)]));
    assert_eq!(device.reset(), Err(Error::I2C(error)));
    device.destroy().done();
}

#[test]
fn reset_ack_is_returned_without_extra_bus_traffic() {
    let mut device = MAX17048::new(Mock::new(&[write(0xfe, 0x5400)]));
    device.reset().unwrap();
    device.destroy().done();
}

#[test]
fn sleep_stops_on_each_failed_step() {
    let sequences = [
        vec![read(0x06, 0).with_error(ErrorKind::Bus)],
        vec![
            read(0x06, 0),
            write(0x06, 0x2000).with_error(ErrorKind::Bus),
        ],
        vec![
            read(0x06, 0),
            write(0x06, 0x2000),
            read(0x0c, 0).with_error(ErrorKind::Bus),
        ],
        vec![
            read(0x06, 0),
            write(0x06, 0x2000),
            read(0x0c, 0x971c),
            write(0x0c, 0x97bc).with_error(ErrorKind::Bus),
        ],
    ];
    for expectations in sequences {
        let mut device = MAX17048::new(Mock::new(&expectations));
        assert_eq!(device.sleep(true), Err(Error::I2C(ErrorKind::Bus)));
        device.destroy().done();
    }
}

fn model_sequence() -> Vec<Transaction> {
    let mut sequence = vec![
        Transaction::write(0x36, vec![0x3f, 0x57]),
        Transaction::write(0x36, vec![0x3e, 0x4a]),
    ];
    for index in 0..32 {
        sequence.push(write(0x40 + 2 * index, 0x1234));
    }
    sequence.push(Transaction::write(0x36, vec![0x3f, 0]));
    sequence.push(Transaction::write(0x36, vec![0x3e, 0]));
    sequence
}

#[test]
fn model_table_transfers_exactly_32_words_and_relocks() {
    let mut device = MAX17048::new(Mock::new(&model_sequence()));
    device
        .write_model_table(&[ModelTableWord::from(0x1234); 32])
        .unwrap();
    device.destroy().done();
}

#[test]
fn model_table_relocks_after_failure_at_every_transfer_step() {
    let complete = model_sequence();
    for failed_index in 0..34 {
        let mut expectations = complete[..=failed_index].to_vec();
        expectations[failed_index] = expectations[failed_index]
            .clone()
            .with_error(ErrorKind::Bus);
        expectations.extend_from_slice(&complete[34..]);
        let mut device = MAX17048::new(Mock::new(&expectations));
        assert_eq!(
            device.write_model_table(&[0x1234.into(); 32]),
            Err(Error::ModelTable {
                operation: Some(ErrorKind::Bus),
                relock_low: None,
                relock_high: None,
            })
        );
        device.destroy().done();
    }
}

#[test]
fn model_table_preserves_transfer_and_both_relock_errors() {
    let mut expectations = vec![model_sequence()[0].clone().with_error(ErrorKind::Bus)];
    expectations
        .push(Transaction::write(0x36, vec![0x3f, 0]).with_error(ErrorKind::ArbitrationLoss));
    expectations.push(Transaction::write(0x36, vec![0x3e, 0]).with_error(ErrorKind::Other));
    let mut device = MAX17048::new(Mock::new(&expectations));
    assert_eq!(
        device.write_model_table(&[0x1234.into(); 32]),
        Err(Error::ModelTable {
            operation: Some(ErrorKind::Bus),
            relock_low: Some(ErrorKind::ArbitrationLoss),
            relock_high: Some(ErrorKind::Other),
        })
    );
    device.destroy().done();
}

#[test]
fn model_table_reports_each_relock_failure_even_after_successful_transfer() {
    for failed_index in [34, 35] {
        let mut expectations = model_sequence();
        expectations[failed_index] = expectations[failed_index]
            .clone()
            .with_error(ErrorKind::Bus);
        let mut device = MAX17048::new(Mock::new(&expectations));
        assert_eq!(
            device.write_model_table(&[0x1234.into(); 32]),
            Err(Error::ModelTable {
                operation: None,
                relock_low: (failed_index == 34).then_some(ErrorKind::Bus),
                relock_high: (failed_index == 35).then_some(ErrorKind::Bus),
            })
        );
        device.destroy().done();
    }
}
