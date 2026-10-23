use embedded_hal::delay::DelayNs;
use embedded_hal_nb::serial::{Read, Write};

use crate::{
    DeviceError,
    comproc::UartTransport,
    config::CommunicationSettings,
    function::RegisterProcessing,
    register::{
        ChopperConfiguration, CurrentControl, DriverStatus, GeneralConfiguration, GlobalStatus,
        InputStatus, InterfaceCounter, MicrostepCurrent, MicrostepResolution, NodeConfiguration,
        OtpConfiguration, OtpProgramming, PwmConfiguration, PwmScaling, ReadableRegister, Register,
        StandstillMode, StepInterval, TargetVelocity, WritableRegister,
    },
    u2, u3, u4, u5, u24,
};

const SILICON_VERSION: u8 = 0x21;
const MAXIMUM_VELOCITY: i32 = 8_388_607;
const OTP_PROGRAMMING_KEY: u8 = 0xbd;
const INTERNAL_SENSE_RESISTANCE_OHMS: f32 = 0.020;

/// TMC2209 UART device; connection only checks identity, never enables or starts a motor.
/// Datasheet rev. 1.09, sections 4-5, 9 and 14; STEP/DIR and ENN remain application-owned.
pub struct TMC2209<UART, DELAY> {
    transport: UartTransport<UART, DELAY>,
    processing: RegisterProcessing,
}

impl<UART: Read<u8> + Write<u8>, DELAY: DelayNs> TMC2209<UART, DELAY> {
    pub fn new(
        uart: UART,
        delay: DELAY,
        settings: CommunicationSettings,
    ) -> Result<Self, DeviceError<UART::Error>> {
        let mut device = Self {
            transport: UartTransport::new(uart, delay, settings)?,
            processing: RegisterProcessing,
        };
        let version = device.read_register::<InputStatus>()?.version();
        if version != SILICON_VERSION {
            return Err(DeviceError::UnsupportedVersion { actual: version }.reported());
        }
        #[cfg(feature = "defmt")]
        defmt::debug!("TMC2209 UART connection verified");
        Ok(device)
    }

    pub fn read_register<R: ReadableRegister>(&mut self) -> Result<R, DeviceError<UART::Error>> {
        self.transport.read(R::ADDRESS).map(R::from)
    }

    /// Write the complete word, then require IFCNT to advance exactly once.
    /// Never retried: an error after transmission may mean the write already took effect.
    pub fn write_register<R: WritableRegister>(
        &mut self,
        register: R,
    ) -> Result<(), DeviceError<UART::Error>> {
        let value = register.into();
        let invalid = value & !R::WRITE_MASK;
        if invalid != 0 {
            return Err(DeviceError::InvalidWriteBits {
                register: R::ADDRESS,
                bits: invalid,
            }
            .reported());
        }
        self.validate_write(R::ADDRESS, value)?;
        self.write_checked(R::ADDRESS, value)
    }

    fn validate_write(&mut self, address: u8, value: u32) -> Result<(), DeviceError<UART::Error>> {
        if address == GeneralConfiguration::ADDRESS {
            let configuration = GeneralConfiguration::from(value);
            if configuration.test_mode() {
                return Err(DeviceError::TestModeEnabled.reported());
            }
            if configuration.step_filter_enabled()
                && self
                    .read_register::<ChopperConfiguration>()?
                    .double_edge_steps()
            {
                return Err(DeviceError::IncompatibleStepFiltering.reported());
            }
        } else if address == ChopperConfiguration::ADDRESS {
            let chopper = ChopperConfiguration::from(value);
            self.processing.microstep_count::<UART::Error>(chopper)?;
            if chopper.off_time().value() == 1 && chopper.blank_time().value() < 2 {
                return Err(DeviceError::InvalidOffTime.reported());
            }
            let hysteresis = u16::from(chopper.hysteresis_start().value())
                + u16::from(chopper.hysteresis_end().value());
            if hysteresis > 18 {
                return Err(DeviceError::InvalidHysteresis.reported());
            }
            if chopper.double_edge_steps()
                && self
                    .read_register::<GeneralConfiguration>()?
                    .step_filter_enabled()
            {
                return Err(DeviceError::IncompatibleStepFiltering.reported());
            }
        } else if address == TargetVelocity::ADDRESS {
            let velocity = ((value << 8) as i32) >> 8;
            if !(-MAXIMUM_VELOCITY..=MAXIMUM_VELOCITY).contains(&velocity) {
                return Err(DeviceError::InvalidVelocity { value: velocity }.reported());
            }
        }
        Ok(())
    }

    fn write_checked(&mut self, address: u8, value: u32) -> Result<(), DeviceError<UART::Error>> {
        let expected = self
            .read_register::<InterfaceCounter>()?
            .count()
            .wrapping_add(1);
        self.transport.write(address, value)?;
        self.verify_write_counter(expected)
    }

    fn verify_write_counter(&mut self, expected: u8) -> Result<(), DeviceError<UART::Error>> {
        let actual = self.read_register::<InterfaceCounter>()?.count();
        if actual != expected {
            return Err(DeviceError::WriteNotAcknowledged { expected, actual }.reported());
        }
        Ok(())
    }

    /// Disable PDN pin current-reduction control and set SENDDELAY, preserving GCONF.
    /// Use delay >=2 for multiple UART nodes; this does not enable output bridges.
    pub fn configure_uart(&mut self, reply_delay: u4) -> Result<(), DeviceError<UART::Error>> {
        let mut configuration = self.read_register::<GeneralConfiguration>()?;
        configuration.set_power_down_pin_disabled(true);
        self.write_register(configuration)?;
        let mut node = NodeConfiguration::from(0);
        node.set_reply_delay(reply_delay);
        self.write_register(node)
    }

    pub fn set_currents(
        &mut self,
        hold: u5,
        run: u5,
        hold_delay: u4,
    ) -> Result<(), DeviceError<UART::Error>> {
        let mut current = CurrentControl::from(0);
        current.set_hold_current(hold);
        current.set_run_current(run);
        current.set_hold_delay(hold_delay);
        self.write_register(current)
    }

    pub fn set_microsteps(
        &mut self,
        resolution: MicrostepResolution,
    ) -> Result<(), DeviceError<UART::Error>> {
        let mut chopper = self.read_register::<ChopperConfiguration>()?;
        let mut configuration = self.read_register::<GeneralConfiguration>()?;
        chopper.set_microstep_resolution(u4::from(resolution));
        self.write_register(chopper)?;
        configuration.set_register_microstep_selection(true);
        self.write_register(configuration)
    }

    pub fn read_microsteps(&mut self) -> Result<u16, DeviceError<UART::Error>> {
        if !self
            .read_register::<GeneralConfiguration>()?
            .register_microstep_selection()
        {
            return Err(DeviceError::PinControlledMicrosteps.reported());
        }
        let chopper = self.read_register::<ChopperConfiguration>()?;
        self.processing.microstep_count(chopper)
    }

    /// Set software bridge enable; hardware ENN can still override this.
    /// A nonzero off time is required, and existing chopper settings are preserved.
    pub fn enable(&mut self, off_time: u4) -> Result<(), DeviceError<UART::Error>> {
        if off_time.value() == 0 {
            return Err(DeviceError::InvalidOffTime.reported());
        }
        let mut chopper = self.read_register::<ChopperConfiguration>()?;
        chopper.set_off_time(off_time);
        self.write_register(chopper)
    }

    /// Set TOFF=0, turning all bridges off; does not change VACTUAL or hardware ENN.
    pub fn disable(&mut self) -> Result<(), DeviceError<UART::Error>> {
        let mut chopper = self.read_register::<ChopperConfiguration>()?;
        chopper.set_off_time(u4::new(0));
        self.write_register(chopper)
    }

    /// Raw VACTUAL units; zero returns control to external STEP/DIR.
    /// Nonzero starts the internal pulse generator if outputs are enabled.
    pub fn set_velocity(&mut self, velocity: i32) -> Result<(), DeviceError<UART::Error>> {
        if !(-MAXIMUM_VELOCITY..=MAXIMUM_VELOCITY).contains(&velocity) {
            return Err(DeviceError::InvalidVelocity { value: velocity }.reported());
        }
        let mut register = TargetVelocity::from(0);
        register.set_signed_velocity_bits(u24::new((velocity as u32) & 0x00ff_ffff));
        self.write_register(register)
    }

    pub fn set_reversed_direction(
        &mut self,
        reversed: bool,
    ) -> Result<(), DeviceError<UART::Error>> {
        let mut configuration = self.read_register::<GeneralConfiguration>()?;
        configuration.set_reversed_direction(reversed);
        self.write_register(configuration)
    }

    /// Only change chopper mode at standstill following the manufacturer tuning procedure.
    /// The SPREAD hardware pin can invert the selected mode.
    pub fn set_spread_cycle(&mut self, enabled: bool) -> Result<(), DeviceError<UART::Error>> {
        let mut configuration = self.read_register::<GeneralConfiguration>()?;
        configuration.set_spread_cycle_enabled(enabled);
        self.write_register(configuration)
    }

    pub fn set_standstill_mode(
        &mut self,
        mode: StandstillMode,
    ) -> Result<(), DeviceError<UART::Error>> {
        let mut configuration = self.read_register::<PwmConfiguration>()?;
        configuration.set_standstill_mode(mode);
        self.write_register(configuration)
    }

    pub fn clear_status_flags(
        &mut self,
        reset: bool,
        driver_error: bool,
    ) -> Result<(), DeviceError<UART::Error>> {
        let mut flags = GlobalStatus::from(0);
        flags.set_reset_detected(reset);
        flags.set_driver_error(driver_error);
        self.write_register(flags)
    }

    /// Return phase A then phase B table values, not measured amperes.
    pub fn read_phase_currents(&mut self) -> Result<(i16, i16), DeviceError<UART::Error>> {
        let register = self.read_register::<MicrostepCurrent>()?;
        Ok(self.processing.phase_currents(register))
    }

    pub fn read_pwm_correction(&mut self) -> Result<i16, DeviceError<UART::Error>> {
        let register = self.read_register::<PwmScaling>()?;
        Ok(self.processing.pwm_correction(register))
    }

    pub fn measured_256_microstep_frequency_hz(&mut self) -> Result<f32, DeviceError<UART::Error>> {
        let interval = self.read_register::<StepInterval>()?;
        let clocks = self.processing.step_clock_period::<UART::Error>(interval)?;
        Ok(self.transport.clock_hz() as f32 / clocks as f32)
    }

    /// Estimate RMS current from CS_ACTUAL and nominal sense voltage.
    /// Requires external sense resistors and disabled analog VREF scaling.
    pub fn estimated_rms_current_amperes(
        &mut self,
        sense_resistance_ohms: f32,
    ) -> Result<f32, DeviceError<UART::Error>> {
        if !sense_resistance_ohms.is_finite() || sense_resistance_ohms <= 0.0 {
            return Err(DeviceError::InvalidSenseResistance.reported());
        }
        let configuration = self.read_register::<GeneralConfiguration>()?;
        if configuration.internal_sense_resistors() || configuration.analog_current_scaling() {
            return Err(DeviceError::UnsupportedCurrentEstimation.reported());
        }
        let chopper = self.read_register::<ChopperConfiguration>()?;
        let status = self.read_register::<DriverStatus>()?;
        Ok(self.processing.rms_sense_voltage(chopper, status)
            / (sense_resistance_ohms + INTERNAL_SENSE_RESISTANCE_OHMS))
    }

    /// Irreversibly set one OTP bit. Never invoked by setup, never retried.
    /// Read PROCEDURE.md and confirm hardware conditions before calling.
    pub fn program_otp_bit(&mut self, byte: u2, bit: u3) -> Result<(), DeviceError<UART::Error>> {
        if byte.value() > 2 {
            return Err(DeviceError::InvalidOtpByte {
                actual: byte.value(),
            }
            .reported());
        }
        let mask = 1_u32 << (u32::from(byte.value()) * 8 + u32::from(bit.value()));
        let previous: u32 = self.read_register::<OtpConfiguration>()?.into();
        if previous & mask != 0 {
            return Err(DeviceError::OtpBitAlreadyProgrammed.reported());
        }
        let mut command = OtpProgramming::from(0);
        command.set_byte_index(byte);
        command.set_bit_index(bit);
        command.set_programming_key(OTP_PROGRAMMING_KEY);
        let expected = self
            .read_register::<InterfaceCounter>()?
            .count()
            .wrapping_add(1);
        self.transport
            .write(OtpProgramming::ADDRESS, command.into())?;
        self.transport.delay_programming();
        self.verify_write_counter(expected)?;
        // Reading OTP_PROG refreshes OTP_READ; the returned word is not a register value.
        self.transport.read(OtpProgramming::ADDRESS)?;
        let actual: u32 = self.read_register::<OtpConfiguration>()?.into();
        if actual & mask == 0 {
            return Err(DeviceError::OtpVerificationFailed {
                byte: byte.value(),
                bit: bit.value(),
            }
            .reported());
        }
        Ok(())
    }

    /// Explicitly discard queued RX bytes after an error; returns the count discarded.
    /// Does not undo a transmitted write or determine its acceptance.
    pub fn recover_uart(&mut self) -> Result<u32, DeviceError<UART::Error>> {
        self.transport.recover()
    }

    pub fn release(self) -> (UART, DELAY) {
        self.transport.release()
    }
}
