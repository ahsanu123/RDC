use embedded_hal::{i2c::I2c, spi::SpiDevice};

use crate::{
    DeviceError,
    comproc::{I2cInterface, Interface, SpiInterface},
    function::PortValues,
    register::{
        Configuration, Direction, GeneralPurposeInputOutput, InputDirection, InputPolarity,
        InterruptCapture, InterruptControl, InterruptDefaultValue, InterruptEnable, InterruptFlags,
        OutputLatch, Pin, Port, PullUpResistors, Register, WritableRegister,
    },
    u3,
};

/// Microchip MCP23017 GPIO expander, generic over its register transport.
///
/// Construction requires the power-on register map (`BANK=0`).
/// Use [`MCP23S17`] for the SPI part. No register values are cached: a failed
/// transaction cannot leave a software shadow disagreeing with hardware.
/// Datasheet: DS20001952D, sections 3.1-3.6, device operation and register access.
pub struct MCP23017<I> {
    interface: I,
    functions: PortValues,
}

/// Microchip MCP23S17 SPI GPIO expander, with the same register API.
pub type MCP23S17<SPI> = MCP23017<SpiInterface<SPI>>;

impl<I2C: I2c> MCP23017<I2cInterface<I2C>> {
    /// Connect to 0x20 + address pins. Does not reset or reconfigure I/O.
    pub fn new_i2c(bus: I2C, address_pins: u3) -> Result<Self, DeviceError<I2C::Error>> {
        Self::connect(I2cInterface::new(bus, address_pins))
    }
}

impl<SPI: SpiDevice<u8>> MCP23017<SpiInterface<SPI>> {
    /// Connect after reset, enabling hardware addressing (including the A2
    /// silicon erratum). Startup affects all reset devices sharing this CS.
    pub fn new_spi(bus: SPI, address_pins: u3) -> Result<Self, DeviceError<SPI::Error>> {
        Self::connect(SpiInterface::new(bus, address_pins))
    }
}

impl<I: Interface> MCP23017<I> {
    pub(crate) fn connect(mut interface: I) -> Result<Self, DeviceError<I::Error>> {
        interface.initialize().map_err(DeviceError::from)?;
        let mut device = Self {
            interface,
            functions: PortValues,
        };
        let configuration = device.read_register::<Configuration>(Port::A)?;
        Self::check_configuration(configuration)?;
        #[cfg(feature = "defmt")]
        defmt::debug!("MCP23x17 connected in BANK=0");
        Ok(device)
    }

    /// Return the owned transport; call its `release` to retrieve the HAL bus.
    pub fn release(self) -> I {
        self.interface
    }

    /// Read a typed register. GPIO and INTCAP reads acknowledge interrupts.
    pub fn read_register<R: Register>(&mut self, port: Port) -> Result<R, DeviceError<I::Error>> {
        self.interface
            .read(R::address(port))
            .map(R::from)
            .map_err(DeviceError::from)
    }

    /// Write a typed writable register. IOCON must use `write_configuration`.
    /// Low-level writes do not check pin capabilities: on MCP23017, never use
    /// GPA7/GPB7 as inputs even though their direction bits reset to one.
    pub fn write_register<R: WritableRegister>(
        &mut self,
        port: Port,
        value: R,
    ) -> Result<(), DeviceError<I::Error>> {
        self.interface
            .write(R::address(port), value.into())
            .map_err(DeviceError::from)
    }

    fn check_configuration(configuration: Configuration) -> Result<(), DeviceError<I::Error>> {
        if configuration.separate_register_banks() {
            #[cfg(feature = "defmt")]
            defmt::error!("MCP23x17 requires BANK=0");
            return Err(DeviceError::UnsupportedRegisterBank);
        }
        if configuration.unimplemented_bit() {
            #[cfg(feature = "defmt")]
            defmt::error!("MCP23x17 unimplemented configuration bit must be zero");
            return Err(DeviceError::ReservedConfigurationBitSet);
        }
        if I::REQUIRES_HAEN && !configuration.hardware_address_enable() {
            #[cfg(feature = "defmt")]
            defmt::error!("MCP23S17 requires hardware addressing");
            return Err(DeviceError::HardwareAddressingDisabled);
        }
        Ok(())
    }

    /// Configure global interrupt and interface behavior without changing the
    /// register map or disabling SPI hardware addressing.
    pub fn write_configuration(
        &mut self,
        configuration: Configuration,
    ) -> Result<(), DeviceError<I::Error>> {
        Self::check_configuration(configuration)?;
        self.interface
            .write(Configuration::ADDRESS_A, configuration.into())
            .map_err(DeviceError::from)
    }

    fn update_bit<R: WritableRegister>(
        &mut self,
        port: Port,
        pin: Pin,
        enabled: bool,
    ) -> Result<(), DeviceError<I::Error>> {
        let previous: u8 = self.read_register::<R>(port)?.into();
        let mask = pin.mask();
        let next = if enabled {
            previous | mask
        } else {
            previous & !mask
        };
        self.write_register(port, R::from(next))
    }

    pub fn set_direction(
        &mut self,
        port: Port,
        pin: Pin,
        direction: Direction,
    ) -> Result<(), DeviceError<I::Error>> {
        if !I::SUPPORTS_PIN7_INPUT && pin == Pin::P7 && direction == Direction::Input {
            #[cfg(feature = "defmt")]
            defmt::error!("MCP23017 pin 7 supports output only");
            return Err(DeviceError::UnsupportedInputPin);
        }
        self.update_bit::<InputDirection>(port, pin, direction == Direction::Input)
    }

    pub fn set_pull_up(
        &mut self,
        port: Port,
        pin: Pin,
        enabled: bool,
    ) -> Result<(), DeviceError<I::Error>> {
        self.update_bit::<PullUpResistors>(port, pin, enabled)
    }

    pub fn set_input_inverted(
        &mut self,
        port: Port,
        pin: Pin,
        inverted: bool,
    ) -> Result<(), DeviceError<I::Error>> {
        self.update_bit::<InputPolarity>(port, pin, inverted)
    }

    pub fn set_interrupt_enabled(
        &mut self,
        port: Port,
        pin: Pin,
        enabled: bool,
    ) -> Result<(), DeviceError<I::Error>> {
        self.update_bit::<InterruptEnable>(port, pin, enabled)
    }

    /// True compares against DEFVAL; false compares against the previous input.
    pub fn set_interrupt_comparison(
        &mut self,
        port: Port,
        pin: Pin,
        compare_to_default: bool,
    ) -> Result<(), DeviceError<I::Error>> {
        self.update_bit::<InterruptControl>(port, pin, compare_to_default)
    }

    pub fn set_interrupt_default(
        &mut self,
        port: Port,
        pin: Pin,
        high: bool,
    ) -> Result<(), DeviceError<I::Error>> {
        self.update_bit::<InterruptDefaultValue>(port, pin, high)
    }

    /// Set a latch bit using OLAT, never GPIO, preserving other output bits.
    /// Set the latch before switching a pin to Output to avoid an initial glitch.
    pub fn set_output(
        &mut self,
        port: Port,
        pin: Pin,
        high: bool,
    ) -> Result<(), DeviceError<I::Error>> {
        self.update_bit::<OutputLatch>(port, pin, high)
    }

    /// Read a pin's GPIO level, acknowledging the entire port's interrupt.
    pub fn read_pin(&mut self, port: Port, pin: Pin) -> Result<bool, DeviceError<I::Error>> {
        let levels: u8 = self
            .read_register::<GeneralPurposeInputOutput>(port)?
            .into();
        Ok(levels & pin.mask() != 0)
    }

    /// Read A then B; A occupies bits 0-7. Acknowledges both interrupts.
    /// The two reads are not an atomic snapshot.
    pub fn read_ports(&mut self) -> Result<u16, DeviceError<I::Error>> {
        let a = self.read_register::<GeneralPurposeInputOutput>(Port::A)?;
        let b = self.read_register::<GeneralPurposeInputOutput>(Port::B)?;
        Ok(self.functions.combine_input_levels(a, b))
    }

    /// Read interrupt captures A then B, acknowledging each port's interrupt.
    /// Read INTF first if the interrupt sources are needed.
    pub fn read_capture(&mut self) -> Result<u16, DeviceError<I::Error>> {
        let a = self.read_register::<InterruptCapture>(Port::A)?;
        let b = self.read_register::<InterruptCapture>(Port::B)?;
        Ok(self.functions.combine_captured_levels(a, b))
    }

    /// Read interrupt-source flags without acknowledging interrupts.
    pub fn read_interrupt_flags(&mut self) -> Result<u16, DeviceError<I::Error>> {
        let a = self.read_register::<InterruptFlags>(Port::A)?;
        let b = self.read_register::<InterruptFlags>(Port::B)?;
        Ok(self.functions.combine_interrupt_flags(a, b))
    }

    /// Read output latches independently of GPIO levels and interrupts.
    pub fn read_output_latches(&mut self) -> Result<u16, DeviceError<I::Error>> {
        let a = self.read_register::<OutputLatch>(Port::A)?;
        let b = self.read_register::<OutputLatch>(Port::B)?;
        Ok(self.functions.combine_output_latches(a, b))
    }

    /// Write A then B, with A in bits 0-7. This is not atomic; if the second
    /// write fails, port A has already changed.
    pub fn write_outputs(&mut self, value: u16) -> Result<(), DeviceError<I::Error>> {
        let [a, b] = value.to_le_bytes();
        self.write_register(Port::A, OutputLatch::from(a))?;
        self.write_register(Port::B, OutputLatch::from(b))
    }
}
