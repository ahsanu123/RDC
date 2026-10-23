use embedded_hal::spi::{ErrorKind, ErrorType, Operation, SpiDevice};
use embedded_hal_mock::eh1::spi::{Mock, Transaction};
use mcp23017::{
    DeviceError, Direction, MCP23S17, Pin, Port,
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
    for (invalid, expected) in [
        (0x00, DeviceError::HardwareAddressingDisabled),
        (0x80, DeviceError::UnsupportedRegisterBank),
        (0x88, DeviceError::UnsupportedRegisterBank),
        (0x09, DeviceError::ReservedConfigurationBitSet),
    ] {
        assert_eq!(
            device.write_configuration(Configuration::from(invalid)),
            Err(expected)
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
        Err(DeviceError::HardwareAddressingDisabled)
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

#[derive(Debug, PartialEq, Eq)]
enum SpiInterfaceError {
    Injected {
        transaction: usize,
        context: Vec<u8>,
    },
    Mock(ErrorKind),
}

impl embedded_hal::spi::Error for SpiInterfaceError {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Injected { .. } => ErrorKind::Other,
            Self::Mock(kind) => *kind,
        }
    }
}

/// Injects serial failures while retaining DS20001952D section 3.2.3 SPI framing.
struct FailingSpi {
    bus: Mock<u8>,
    transactions_before_failure: usize,
    failure: Option<SpiInterfaceError>,
}

impl ErrorType for FailingSpi {
    type Error = SpiInterfaceError;
}

impl SpiDevice for FailingSpi {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        if self.transactions_before_failure == 0 {
            return Err(self
                .failure
                .take()
                .expect("failure must only be injected once"));
        }
        self.transactions_before_failure -= 1;
        self.bus
            .transaction(operations)
            .map_err(SpiInterfaceError::Mock)
    }
}

fn injected_failure(transaction: usize) -> SpiInterfaceError {
    SpiInterfaceError::Injected {
        transaction,
        context: vec![0xde, 0xad],
    }
}

#[test]
fn test_spi_connection_forwards_owned_errors_from_every_initialization_step() {
    for (transaction, completed_expectations) in [(0, 0), (1, 3), (2, 6)] {
        let expectations = initialization(5);
        let mut bus = Mock::new(&expectations[..completed_expectations]);
        let interface = SpiInterface::new(
            FailingSpi {
                bus: bus.clone(),
                transactions_before_failure: transaction,
                failure: Some(injected_failure(transaction)),
            },
            u3::new(5),
        );
        let result = Builder::builder().interface(interface).build().connect();
        assert!(matches!(
            result,
            Err(DeviceError::Interface(error)) if error == injected_failure(transaction)
        ));
        bus.done();
    }
}

#[test]
fn test_spi_register_and_configuration_operations_forward_owned_errors() {
    for operation in 0..3 {
        let mut bus = Mock::new(&initialization(5));
        let mut device = MCP23S17::new_spi(
            FailingSpi {
                bus: bus.clone(),
                transactions_before_failure: 3,
                failure: Some(injected_failure(3)),
            },
            u3::new(5),
        )
        .unwrap();
        let result = match operation {
            0 => device
                .read_register::<InterruptCapture>(Port::B)
                .map(|_| ()),
            1 => device.write_register(Port::B, OutputLatch::from(0x81)),
            _ => device.write_configuration(Configuration::from(0x0c)),
        };
        assert_eq!(result, Err(DeviceError::Interface(injected_failure(3))));
        bus.done();
    }
}
