use embedded_hal::i2c::{ErrorKind, NoAcknowledgeSource};
use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
use mcp23017::{
    Direction, Error, MCP23017, Pin, Port,
    builder::Builder,
    comproc::I2cInterface,
    register::{
        Configuration, GeneralPurposeInputOutput, InputDirection, InputPolarity, InterruptCapture,
        InterruptControl, InterruptDefaultValue, InterruptEnable, InterruptFlags, OutputLatch,
        PullUpResistors, Register,
    },
    u3,
};

fn connected(bus: Mock) -> MCP23017<I2cInterface<Mock>> {
    MCP23017::new_i2c(bus, u3::new(0)).unwrap()
}

#[test]
fn test_i2c_builder_uses_all_address_pin_combinations() {
    for pins in 0..8 {
        let expectations = [
            Transaction::write_read(0x20 + pins, vec![0x0a], vec![0]),
            Transaction::write(0x20 + pins, vec![0x01, 0x3c]),
        ];
        let bus = Mock::new(&expectations);
        let mut device = Builder::builder()
            .interface(I2cInterface::new(bus.clone(), u3::new(pins)))
            .build()
            .connect()
            .unwrap();
        device
            .write_register(Port::B, InputDirection::from(0x3c))
            .unwrap();
        device.release().release().done();
    }
}

fn check_register_read<R: Register>(device: &mut MCP23017<I2cInterface<Mock>>, expected: u8) {
    let a: u8 = device.read_register::<R>(Port::A).unwrap().into();
    let b: u8 = device.read_register::<R>(Port::B).unwrap().into();
    assert_eq!((a, b), (expected, expected ^ 0xff));
}

#[test]
fn test_i2c_reads_every_typed_register_at_both_port_addresses() {
    let mut expectations = vec![Transaction::write_read(0x20, vec![0x0a], vec![0])];
    for address in (0..=0x14).step_by(2) {
        expectations.push(Transaction::write_read(
            0x20,
            vec![address],
            vec![address + 0x20],
        ));
        expectations.push(Transaction::write_read(
            0x20,
            vec![address + 1],
            vec![(address + 0x20) ^ 0xff],
        ));
    }
    let mut bus = Mock::new(&expectations);
    let mut device = connected(bus.clone());
    check_register_read::<InputDirection>(&mut device, 0x20);
    check_register_read::<InputPolarity>(&mut device, 0x22);
    check_register_read::<InterruptEnable>(&mut device, 0x24);
    check_register_read::<InterruptDefaultValue>(&mut device, 0x26);
    check_register_read::<InterruptControl>(&mut device, 0x28);
    check_register_read::<Configuration>(&mut device, 0x2a);
    check_register_read::<PullUpResistors>(&mut device, 0x2c);
    check_register_read::<InterruptFlags>(&mut device, 0x2e);
    check_register_read::<InterruptCapture>(&mut device, 0x30);
    check_register_read::<GeneralPurposeInputOutput>(&mut device, 0x32);
    check_register_read::<OutputLatch>(&mut device, 0x34);
    bus.done();
}

#[test]
fn test_direction_preserves_other_pins_and_uses_correct_port() {
    let expectations = [
        Transaction::write_read(0x20, vec![0x0a], vec![0]),
        Transaction::write_read(0x20, vec![0x00], vec![0xff]),
        Transaction::write(0x20, vec![0x00, 0xfe]),
        Transaction::write_read(0x20, vec![0x01], vec![0x82]),
        Transaction::write(0x20, vec![0x01, 0xc2]),
    ];
    let mut bus = Mock::new(&expectations);
    let mut device = connected(bus.clone());
    device
        .set_direction(Port::A, Pin::P0, Direction::Output)
        .unwrap();
    device
        .set_direction(Port::B, Pin::P6, Direction::Input)
        .unwrap();
    bus.done();
}

#[test]
fn test_output_updates_latch_without_reading_gpio() {
    let expectations = [
        Transaction::write_read(0x20, vec![0x0a], vec![0]),
        Transaction::write_read(0x20, vec![0x15], vec![0xa5]),
        Transaction::write(0x20, vec![0x15, 0xa7]),
        Transaction::write_read(0x20, vec![0x15], vec![0xa7]),
        Transaction::write(0x20, vec![0x15, 0x27]),
    ];
    let mut bus = Mock::new(&expectations);
    let mut device = connected(bus.clone());
    device.set_output(Port::B, Pin::P1, true).unwrap();
    device.set_output(Port::B, Pin::P7, false).unwrap();
    bus.done();
}

#[test]
fn test_pull_up_polarity_and_interrupt_configuration_preserve_neighbors() {
    let expectations = [
        Transaction::write_read(0x20, vec![0x0a], vec![0]),
        Transaction::write_read(0x20, vec![0x0d], vec![0x01]),
        Transaction::write(0x20, vec![0x0d, 0x41]),
        Transaction::write_read(0x20, vec![0x02], vec![0x01]),
        Transaction::write(0x20, vec![0x02, 0x03]),
        Transaction::write_read(0x20, vec![0x04], vec![0xff]),
        Transaction::write(0x20, vec![0x04, 0xef]),
        Transaction::write_read(0x20, vec![0x09], vec![0x80]),
        Transaction::write(0x20, vec![0x09, 0x84]),
        Transaction::write_read(0x20, vec![0x07], vec![0x80]),
        Transaction::write(0x20, vec![0x07, 0x81]),
    ];
    let mut bus = Mock::new(&expectations);
    let mut device = connected(bus.clone());
    device.set_pull_up(Port::B, Pin::P6, true).unwrap();
    device.set_input_inverted(Port::A, Pin::P1, true).unwrap();
    device
        .set_interrupt_enabled(Port::A, Pin::P4, false)
        .unwrap();
    device
        .set_interrupt_comparison(Port::B, Pin::P2, true)
        .unwrap();
    device
        .set_interrupt_default(Port::B, Pin::P0, true)
        .unwrap();
    bus.done();
}

#[test]
fn test_combined_reads_use_independent_addresses_with_sequential_mode_disabled() {
    let expectations = [
        Transaction::write_read(0x20, vec![0x0a], vec![0x20]),
        Transaction::write_read(0x20, vec![0x12], vec![0x81]),
        Transaction::write_read(0x20, vec![0x13], vec![0x42]),
        Transaction::write_read(0x20, vec![0x0e], vec![0x01]),
        Transaction::write_read(0x20, vec![0x0f], vec![0x80]),
        Transaction::write_read(0x20, vec![0x10], vec![0xa5]),
        Transaction::write_read(0x20, vec![0x11], vec![0x5a]),
        Transaction::write_read(0x20, vec![0x14], vec![0x34]),
        Transaction::write_read(0x20, vec![0x15], vec![0x12]),
    ];
    let mut bus = Mock::new(&expectations);
    let mut device = connected(bus.clone());
    assert_eq!(device.read_ports().unwrap(), 0x4281);
    assert_eq!(device.read_interrupt_flags().unwrap(), 0x8001);
    assert_eq!(device.read_capture().unwrap(), 0x5aa5);
    assert_eq!(device.read_output_latches().unwrap(), 0x1234);
    bus.done();
}

#[test]
fn test_read_pin_masks_requested_bit_at_both_boundaries() {
    let expectations = [
        Transaction::write_read(0x20, vec![0x0a], vec![0]),
        Transaction::write_read(0x20, vec![0x12], vec![0x01]),
        Transaction::write_read(0x20, vec![0x12], vec![0x01]),
        Transaction::write_read(0x20, vec![0x13], vec![0x80]),
        Transaction::write_read(0x20, vec![0x13], vec![0x80]),
    ];
    let mut bus = Mock::new(&expectations);
    let mut device = connected(bus.clone());
    assert!(device.read_pin(Port::A, Pin::P0).unwrap());
    assert!(!device.read_pin(Port::A, Pin::P7).unwrap());
    assert!(device.read_pin(Port::B, Pin::P7).unwrap());
    assert!(!device.read_pin(Port::B, Pin::P0).unwrap());
    bus.done();
}

#[test]
fn test_write_outputs_uses_port_a_as_low_byte() {
    let expectations = [
        Transaction::write_read(0x20, vec![0x0a], vec![0]),
        Transaction::write(0x20, vec![0x14, 0x34]),
        Transaction::write(0x20, vec![0x15, 0x12]),
    ];
    let mut bus = Mock::new(&expectations);
    connected(bus.clone()).write_outputs(0x1234).unwrap();
    bus.done();
}

#[test]
fn test_configuration_rejects_unsafe_bits_without_bus_writes() {
    let expectations = [Transaction::write_read(0x20, vec![0x0a], vec![0])];
    let mut bus = Mock::new(&expectations);
    let mut device = connected(bus.clone());
    for invalid in [0x80, 0x01, 0x81] {
        assert_eq!(
            device.write_configuration(Configuration::from(invalid)),
            Err(Error::InvalidConfiguration)
        );
    }
    bus.done();
}

#[test]
fn test_configuration_preserves_interrupt_and_interface_settings() {
    let expectations = [
        Transaction::write_read(0x20, vec![0x0a], vec![0x60]),
        Transaction::write(0x20, vec![0x0a, 0x64]),
    ];
    let mut bus = Mock::new(&expectations);
    let mut device = connected(bus.clone());
    let mut configuration = Configuration::from(0x60);
    configuration.set_interrupt_open_drain(true);
    device.write_configuration(configuration).unwrap();
    bus.done();
}

#[test]
fn test_connection_propagates_read_error() {
    let failure = ErrorKind::NoAcknowledge(NoAcknowledgeSource::Address);
    let expectations = [Transaction::write_read(0x20, vec![0x0a], vec![0]).with_error(failure)];
    let mut bus = Mock::new(&expectations);
    assert!(matches!(
        MCP23017::new_i2c(bus.clone(), u3::new(0)),
        Err(Error::Bus(error)) if error == failure
    ));
    bus.done();
}

#[test]
fn test_failed_update_read_does_not_write() {
    let expectations = [
        Transaction::write_read(0x20, vec![0x0a], vec![0]),
        Transaction::write_read(0x20, vec![0x14], vec![0]).with_error(ErrorKind::Bus),
    ];
    let mut bus = Mock::new(&expectations);
    assert_eq!(
        connected(bus.clone()).set_output(Port::A, Pin::P0, true),
        Err(Error::Bus(ErrorKind::Bus))
    );
    bus.done();
}

#[test]
fn test_failed_update_write_is_returned_and_next_update_rereads_hardware() {
    let expectations = [
        Transaction::write_read(0x20, vec![0x0a], vec![0]),
        Transaction::write_read(0x20, vec![0x14], vec![0x80]),
        Transaction::write(0x20, vec![0x14, 0x81]).with_error(ErrorKind::Bus),
        Transaction::write_read(0x20, vec![0x14], vec![0x40]),
        Transaction::write(0x20, vec![0x14, 0x42]),
    ];
    let mut bus = Mock::new(&expectations);
    let mut device = connected(bus.clone());
    assert_eq!(
        device.set_output(Port::A, Pin::P0, true),
        Err(Error::Bus(ErrorKind::Bus))
    );
    device.set_output(Port::A, Pin::P1, true).unwrap();
    bus.done();
}

#[test]
fn test_combined_read_returns_error_instead_of_partial_value() {
    let expectations = [
        Transaction::write_read(0x20, vec![0x0a], vec![0]),
        Transaction::write_read(0x20, vec![0x10], vec![0x81]),
        Transaction::write_read(0x20, vec![0x11], vec![0]).with_error(ErrorKind::Bus),
    ];
    let mut bus = Mock::new(&expectations);
    assert_eq!(
        connected(bus.clone()).read_capture(),
        Err(Error::Bus(ErrorKind::Bus))
    );
    bus.done();
}

#[test]
fn test_combined_write_stops_after_first_failure() {
    let expectations = [
        Transaction::write_read(0x20, vec![0x0a], vec![0]),
        Transaction::write(0x20, vec![0x14, 0x34]).with_error(ErrorKind::Bus),
    ];
    let mut bus = Mock::new(&expectations);
    assert_eq!(
        connected(bus.clone()).write_outputs(0x1234),
        Err(Error::Bus(ErrorKind::Bus))
    );
    bus.done();
}

#[test]
fn test_combined_write_reports_failure_after_port_a_is_written() {
    let expectations = [
        Transaction::write_read(0x20, vec![0x0a], vec![0]),
        Transaction::write(0x20, vec![0x14, 0x34]),
        Transaction::write(0x20, vec![0x15, 0x12]).with_error(ErrorKind::Bus),
    ];
    let mut bus = Mock::new(&expectations);
    assert_eq!(
        connected(bus.clone()).write_outputs(0x1234),
        Err(Error::Bus(ErrorKind::Bus))
    );
    bus.done();
}

#[test]
fn test_i2c_rejects_input_direction_on_output_only_pins_without_io() {
    let expectations = [Transaction::write_read(0x20, vec![0x0a], vec![0])];
    let mut bus = Mock::new(&expectations);
    let mut device = connected(bus.clone());
    for port in [Port::A, Port::B] {
        assert_eq!(
            device.set_direction(port, Pin::P7, Direction::Input),
            Err(Error::UnsupportedInputPin)
        );
    }
    bus.done();
}
