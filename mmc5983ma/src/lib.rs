#![no_std]
#![doc = include_str!("../README.md")]

pub mod builder;
pub mod comproc;
mod error;
mod function;
pub mod register;

use core::num::NonZeroU16;
use embedded_hal::delay::DelayNs;
use function::{MagneticConversion, OffsetCompensation, TemperatureConversion};
use register::*;

pub use error::{ConfigurationError, Error};

const STARTUP_DELAY_MS: u32 = 10;
// Rev A specifies a 500 ns pulse but no host-command settling interval.
// A conservative 1 ms delay is used; this is not a manufacturer maximum.
const PULSE_SETTLING_DELAY_MS: u32 = 1;
const POLL_DELAY_MS: u32 = 1;
const CONTROL0_PERSISTENT_MASK: u8 = 0x24;

/// Nominal field strength in gauss, aligned to the datasheet's post-SET axes.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MagneticField {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl MagneticField {
    fn from_array([x, y, z]: [f32; 3]) -> Self {
        Self { x, y, z }
    }
}

/// Unsigned offset-binary counts and nominal physical field.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MagneticMeasurement {
    pub raw: [u32; 3],
    pub gauss: MagneticField,
}

/// Stable control register settings. Command bits must be zero.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Configuration {
    pub control0: Control0,
    pub control1: Control1,
    pub control2: Control2,
    pub control3: Control3,
}

impl Configuration {
    pub fn validate(&self) -> Result<(), ConfigurationError> {
        use ConfigurationError::*;
        let c0 = u8::from(self.control0);
        let c1 = u8::from(self.control1);
        let c3 = u8::from(self.control3);
        if c0 & 0x80 != 0 || c1 & 0x60 != 0 || c3 & 0xb9 != 0 {
            return Err(ReservedBits);
        }
        if c0 & !CONTROL0_PERSISTENT_MASK != 0 || self.control1.software_reset() {
            return Err(CommandBits);
        }
        if !matches!(self.control1.yz_inhibited().value(), 0 | 3) {
            return Err(InvalidChannelInhibit);
        }
        if self.control3.three_wire_spi_enabled() {
            return Err(ThreeWireSpiUnsupported);
        }
        if self.control3.positive_self_test_enabled() && self.control3.negative_self_test_enabled()
        {
            return Err(OpposingSelfTestCurrents);
        }
        if self.control2.continuous_measurement_enabled() {
            let valid = match self.control2.frequency() {
                ContinuousFrequency::Off => return Err(InvalidContinuousFrequency),
                ContinuousFrequency::Hertz100 => self.control1.bandwidth() != Bandwidth::Hertz100,
                ContinuousFrequency::Hertz200 => self.control1.bandwidth() == Bandwidth::Hertz200,
                ContinuousFrequency::Hertz1000 => self.control1.bandwidth() == Bandwidth::Hertz800,
                _ => true,
            };
            if !valid {
                return Err(InsufficientBandwidth);
            }
        }
        if self.control2.periodic_set_enabled()
            && !(self.control0.automatic_set_reset_enabled()
                && self.control2.continuous_measurement_enabled())
        {
            return Err(PeriodicSetRequiresAutomaticAndContinuous);
        }
        Ok(())
    }
}

/// MEMSIC MMC5983MA (Rev A). Owns a protocol adapter and write-only control state.
pub struct Mmc5983ma<Interface> {
    interface: Interface,
    configuration: Configuration,
    initialized: bool,
    poll_limit: NonZeroU16,
    magnetic_conversion: MagneticConversion,
    temperature_conversion: TemperatureConversion,
    offset_compensation: OffsetCompensation,
}

impl<Interface: comproc::RegisterInterface> Mmc5983ma<Interface> {
    /// Construct without I/O. Call initialize after power is stable.
    pub fn new(interface: Interface, poll_limit: NonZeroU16) -> Self {
        Self {
            interface,
            configuration: Configuration::default(),
            initialized: false,
            poll_limit,
            magnetic_conversion: MagneticConversion,
            temperature_conversion: TemperatureConversion,
            offset_compensation: OffsetCompensation,
        }
    }

    /// Return the owned adapter without issuing bus commands.
    pub fn release(self) -> Interface {
        self.interface
    }

    /// Power-up wait, identity check, software reset, OTP check and initial SET.
    /// This also recovers from uncertain state following a failed write.
    pub fn initialize(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<Interface::Error>> {
        self.initialized = false;
        delay.delay_ms(STARTUP_DELAY_MS);
        let product = self.read_product_id()?.value();
        if product != PRODUCT_ID {
            return Err(Error::UnexpectedProductId(product));
        }
        self.write(Address::Control1, 0x80)?;
        delay.delay_ms(STARTUP_DELAY_MS);
        self.configuration = Configuration::default();
        if !self.read_status()?.otp_read_done() {
            return Err(Error::OtpNotReady);
        }
        self.write(Address::Control0, 0x08)?;
        delay.delay_ms(PULSE_SETTLING_DELAY_MS);
        self.initialized = true;
        #[cfg(feature = "defmt")]
        defmt::debug!("MMC5983MA initialized");
        Ok(())
    }

    pub fn read_product_id(&mut self) -> Result<OutputByte, Error<Interface::Error>> {
        self.read_byte(Address::ProductIdentification)
            .map(Into::into)
    }

    pub fn read_status(&mut self) -> Result<Status, Error<Interface::Error>> {
        self.read_byte(Address::Status).map(Into::into)
    }

    /// Return tracked write-only controls, not a hardware readback.
    pub fn configuration(&self) -> Result<Configuration, Error<Interface::Error>> {
        self.ensure_initialized()?;
        Ok(self.configuration)
    }

    /// Apply stable controls after disabling continuous mode.
    /// A write failure requires reinitialization; no rollback is attempted.
    pub fn configure(
        &mut self,
        configuration: Configuration,
        delay: &mut impl DelayNs,
    ) -> Result<(), Error<Interface::Error>> {
        self.ensure_initialized()?;
        configuration
            .validate()
            .map_err(Error::InvalidConfiguration)?;
        let mut stopped = self.configuration.control2;
        stopped.set_continuous_measurement_enabled(false);
        stopped.set_periodic_set_enabled(false);
        self.write(Address::Control2, stopped.into())?;
        self.configuration.control2 = stopped;
        // Allow an in-flight conversion to finish before changing its settings.
        delay.delay_ms(STARTUP_DELAY_MS);
        self.write(Address::Control0, configuration.control0.into())?;
        self.configuration.control0 = configuration.control0;
        self.write(Address::Control1, configuration.control1.into())?;
        self.configuration.control1 = configuration.control1;
        self.write(Address::Control3, configuration.control3.into())?;
        self.configuration.control3 = configuration.control3;
        self.write(Address::Control2, configuration.control2.into())?;
        self.configuration.control2 = configuration.control2;
        Ok(())
    }

    /// Acknowledge selected measurement interrupts with a W1C write.
    pub fn clear_interrupts(
        &mut self,
        magnetic: bool,
        temperature: bool,
    ) -> Result<(), Error<Interface::Error>> {
        self.ensure_initialized()?;
        self.write(
            Address::Status,
            u8::from(magnetic) | (u8::from(temperature) << 1),
        )
    }

    /// Apply a SET pulse and wait a conservative 1 ms.
    pub fn set(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<Interface::Error>> {
        self.ensure_single_shot()?;
        self.command(0x08)?;
        delay.delay_ms(PULSE_SETTLING_DELAY_MS);
        Ok(())
    }

    /// Apply a RESET pulse (reverses sensing polarity) and wait a conservative 1 ms.
    pub fn reset(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<Interface::Error>> {
        self.ensure_single_shot()?;
        self.command(0x10)?;
        delay.delay_ms(PULSE_SETTLING_DELAY_MS);
        Ok(())
    }

    /// Reload OTP calibration and verify completion using the bounded poll limit.
    pub fn reload_otp(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<Interface::Error>> {
        self.ensure_single_shot()?;
        self.command(0x40)?;
        // OTP completion may still be asserted when the command is accepted.
        delay.delay_ms(STARTUP_DELAY_MS);
        self.wait_ready(0x10, delay)
    }

    /// Read an already completed magnetic measurement; no conversion is started.
    /// Continuous-mode data can change during the seven-byte burst.
    pub fn read_magnetic_field(&mut self) -> Result<MagneticMeasurement, Error<Interface::Error>> {
        self.ensure_magnetic_channels()?;
        if !self.read_status()?.magnetic_measurement_done() {
            return Err(Error::MeasurementNotReady);
        }
        let registers = self.read_magnetic_registers()?;
        Ok(self.magnetic_conversion.convert(registers))
    }

    /// Read an already completed temperature measurement in degrees Celsius.
    pub fn read_temperature(&mut self) -> Result<f32, Error<Interface::Error>> {
        self.ensure_initialized()?;
        if !self.read_status()?.temperature_measurement_done() {
            return Err(Error::MeasurementNotReady);
        }
        let register = self.read_byte(Address::Temperature)?.into();
        Ok(self.temperature_conversion.convert(register))
    }

    /// Trigger one magnetic conversion, poll with a bound and read all seven bytes.
    pub fn measure_magnetic_field(
        &mut self,
        delay: &mut impl DelayNs,
    ) -> Result<MagneticMeasurement, Error<Interface::Error>> {
        let registers = self.acquire_magnetic(delay)?;
        Ok(self.magnetic_conversion.convert(registers))
    }

    /// Trigger one temperature conversion, poll with a bound and return Celsius.
    pub fn measure_temperature(
        &mut self,
        delay: &mut impl DelayNs,
    ) -> Result<f32, Error<Interface::Error>> {
        self.ensure_single_shot()?;
        self.clear_interrupts(false, true)?;
        self.command(0x02)?;
        self.wait_ready(0x02, delay)?;
        let register = self
            .read_byte(Address::Temperature)
            .inspect_err(|_| {
                self.initialized = false;
            })?
            .into();
        Ok(self.temperature_conversion.convert(register))
    }

    /// Remove bridge offset using SET and RESET measurements, restoring SET last.
    /// Auto S/R must be disabled. Failure may leave RESET polarity active.
    pub fn measure_offset_compensated(
        &mut self,
        delay: &mut impl DelayNs,
    ) -> Result<MagneticField, Error<Interface::Error>> {
        self.ensure_single_shot()?;
        self.ensure_magnetic_channels()?;
        if self.configuration.control0.automatic_set_reset_enabled() {
            return Err(Error::AutomaticSetResetActive);
        }
        let result = (|| {
            self.set(delay)?;
            let set = self.acquire_magnetic(delay)?;
            self.reset(delay)?;
            let reset = self.acquire_magnetic(delay)?;
            self.set(delay)?;
            Ok(self.offset_compensation.convert(set, reset))
        })();
        if result.is_err() {
            self.initialized = false;
        }
        result
    }

    fn acquire_magnetic(
        &mut self,
        delay: &mut impl DelayNs,
    ) -> Result<MagneticOutputRegisters, Error<Interface::Error>> {
        self.ensure_single_shot()?;
        self.ensure_magnetic_channels()?;
        self.clear_interrupts(true, false)?;
        self.command(0x01)?;
        self.wait_ready(0x01, delay)?;
        self.read_magnetic_registers().inspect_err(|_| {
            self.initialized = false;
        })
    }

    fn read_magnetic_registers(
        &mut self,
    ) -> Result<MagneticOutputRegisters, Error<Interface::Error>> {
        let mut bytes = [0; 7];
        self.interface
            .read(Address::XOutputHigh, &mut bytes)
            .map_err(Error::Interface)?;
        Ok(bytes.into())
    }

    fn wait_ready(
        &mut self,
        mask: u8,
        delay: &mut impl DelayNs,
    ) -> Result<(), Error<Interface::Error>> {
        for _ in 0..self.poll_limit.get() {
            delay.delay_ms(POLL_DELAY_MS);
            let status = self.read_status().inspect_err(|_| {
                self.initialized = false;
            })?;
            if u8::from(status) & mask != 0 {
                return Ok(());
            }
        }
        // A conversion may still be active; prevent another command racing it.
        self.initialized = false;
        Err(Error::MeasurementTimeout)
    }

    fn ensure_initialized(&self) -> Result<(), Error<Interface::Error>> {
        if self.initialized {
            Ok(())
        } else {
            Err(Error::NotInitialized)
        }
    }

    fn ensure_single_shot(&self) -> Result<(), Error<Interface::Error>> {
        self.ensure_initialized()?;
        if self.configuration.control2.continuous_measurement_enabled() {
            Err(Error::ContinuousModeActive)
        } else {
            Ok(())
        }
    }

    fn ensure_magnetic_channels(&self) -> Result<(), Error<Interface::Error>> {
        self.ensure_initialized()?;
        if self.configuration.control1.x_inhibited()
            || self.configuration.control1.yz_inhibited().value() != 0
        {
            Err(Error::ChannelsDisabled)
        } else {
            Ok(())
        }
    }

    fn command(&mut self, command: u8) -> Result<(), Error<Interface::Error>> {
        self.write(
            Address::Control0,
            (u8::from(self.configuration.control0) & CONTROL0_PERSISTENT_MASK) | command,
        )
    }

    fn read_byte(&mut self, address: Address) -> Result<u8, Error<Interface::Error>> {
        let mut byte = [0];
        self.interface
            .read(address, &mut byte)
            .map_err(Error::Interface)?;
        Ok(byte[0])
    }

    fn write(&mut self, address: Address, value: u8) -> Result<(), Error<Interface::Error>> {
        self.interface.write(address, value).map_err(|error| {
            self.initialized = false;
            Error::Interface(error)
        })
    }
}
