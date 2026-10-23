use embedded_hal_mock::eh1::spi::{Mock, Transaction};
use mcp23017::{
    Direction, Error, MCP23S17, Pin, Port,
    builder::Builder,
    comproc::SpiInterface,
    register::{Configuration, InterruptCapture, OutputLatch},
    u3,
};

fn initialization(address_pins: u8) -> Vec<Transaction<u8>> {
    vec![
        Transaction::transaction_start(),
        Transaction::write_vec(vec![0x40, 0x0a, 0x08]),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::write_vec(vec![0x48, 0x0a, 0x08]),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::write_vec(vec![0x41 | (address_pins << 1), 0x0a]),
        Transaction::read_vec(vec![0x08]),
        Transaction::transaction_end(),
    ]
}

#[test]
fn test_spi_initialization_covers_erratum_and_selects_all_hardware_addresses() {
    for pins in 0..8 {
        let mut expectations = initialization(pins);
        expectations.extend([
            Transaction::transaction_start(),
            Transaction::write_vec(vec![0x40 | (pins << 1), 0x15, 0xa5]),
            Transaction::transaction_end(),
        ]);
        let bus = Mock::new(&expectations);
        let mut device = MCP23S17::new_spi(bus.clone(), u3::new(pins)).unwrap();
        device
            .write_register(Port::B, OutputLatch::from(0xa5))
            .unwrap();
        device.release().release().done();
    }
}

#[test]
fn test_spi_builder_reads_capture_in_one_cs_transaction() {
    let mut expectations = initialization(5);
    expectations.extend([
        Transaction::transaction_start(),
        Transaction::write_vec(vec![0x4b, 0x11]),
        Transaction::read_vec(vec![0x81]),
        Transaction::transaction_end(),
    ]);
    let mut bus = Mock::new(&expectations);
    let mut device = Builder::builder()
        .interface(SpiInterface::new(bus.clone(), u3::new(5)))
        .build()
        .connect()
        .unwrap();
    let capture = device.read_register::<InterruptCapture>(Port::B).unwrap();
    assert!(capture.pin7());
    assert!(capture.pin0());
    assert!(!capture.pin1());
    bus.done();
}

#[test]
fn test_spi_output_update_uses_selected_address_and_latch() {
    let mut expectations = initialization(7);
    expectations.extend([
        Transaction::transaction_start(),
        Transaction::write_vec(vec![0x4f, 0x14]),
        Transaction::read_vec(vec![0x80]),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::write_vec(vec![0x4e, 0x14, 0x81]),
        Transaction::transaction_end(),
    ]);
    let mut bus = Mock::new(&expectations);
    let mut device = MCP23S17::new_spi(bus.clone(), u3::new(7)).unwrap();
    device.set_output(Port::A, Pin::P0, true).unwrap();
    bus.done();
}

#[test]
fn test_spi_configuration_rejects_disabled_hardware_addressing_before_io() {
    let mut bus = Mock::new(&initialization(2));
    let mut device = MCP23S17::new_spi(bus.clone(), u3::new(2)).unwrap();
    for invalid in [0x00, 0x80, 0x88, 0x09] {
        assert_eq!(
            device.write_configuration(Configuration::from(invalid)),
            Err(Error::InvalidConfiguration)
        );
    }
    bus.done();
}

#[test]
fn test_spi_configuration_keeps_addressing_enabled() {
    let mut expectations = initialization(3);
    expectations.extend([
        Transaction::transaction_start(),
        Transaction::write_vec(vec![0x46, 0x0a, 0x6c]),
        Transaction::transaction_end(),
    ]);
    let mut bus = Mock::new(&expectations);
    let mut device = MCP23S17::new_spi(bus.clone(), u3::new(3)).unwrap();
    device
        .write_configuration(Configuration::from(0x6c))
        .unwrap();
    bus.done();
}

#[test]
fn test_spi_connection_rejects_failed_hardware_address_enable_readback() {
    let mut expectations = initialization(4);
    expectations[8] = Transaction::read_vec(vec![0x00]);
    let mut bus = Mock::new(&expectations);
    assert!(matches!(
        MCP23S17::new_spi(bus.clone(), u3::new(4)),
        Err(Error::InvalidConfiguration)
    ));
    bus.done();
}

#[test]
fn test_spi_allows_pin7_input_direction() {
    let mut expectations = initialization(0);
    expectations.extend([
        Transaction::transaction_start(),
        Transaction::write_vec(vec![0x41, 0x01]),
        Transaction::read_vec(vec![0x00]),
        Transaction::transaction_end(),
        Transaction::transaction_start(),
        Transaction::write_vec(vec![0x40, 0x01, 0x80]),
        Transaction::transaction_end(),
    ]);
    let mut bus = Mock::new(&expectations);
    let mut device = MCP23S17::new_spi(bus.clone(), u3::new(0)).unwrap();
    device
        .set_direction(Port::B, Pin::P7, Direction::Input)
        .unwrap();
    bus.done();
}
