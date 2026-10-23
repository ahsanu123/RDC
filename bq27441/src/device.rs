use embedded_hal::{delay::DelayNs, i2c::I2c};

use crate::{
    comproc::I2cInterface, error::DeviceError, function::Measurements, register::*, validation,
};

const CONTROL_ADDRESS: u8 = 0x00;
const DATA_CLASS_ADDRESS: u8 = 0x3e;
const DATA_BLOCK_ADDRESS: u8 = 0x3f;
const BLOCK_DATA_ADDRESS: u8 = 0x40;
const BLOCK_CHECKSUM_ADDRESS: u8 = 0x60;
const BLOCK_CONTROL_ADDRESS: u8 = 0x61;
const BLOCK_SELECTION_DELAY_MS: u32 = 1;
const BLOCK_COMMIT_DELAY_MS: u32 = 10;
const POLL_ATTEMPTS: usize = 4;
const UNSEAL_WORD: u16 = 0x8000;

/// Fixed ROM chemistry variants; SLUSBH1C device comparison table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Variant {
    /// 4.2 V charge voltage.
    G1A,
    /// 4.3 V or 4.35 V charge voltage.
    G1B,
}

/// Cell-specific State-subclass parameters; SLUUAC9A sections 3.1 and 6.4.2.3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BatteryConfiguration {
    pub design_capacity_milliamp_hours: u16,
    pub design_energy_milliwatt_hours: u16,
    pub terminate_voltage_millivolts: u16,
    /// 10 * design capacity (mAh) / charge termination current (mA).
    pub taper_rate: u16,
}

/// BQ27441-G1 fuel gauge; owns its I2C handle, delay and pure processing helper.
///
/// Measurements are read on demand, not cached. The application must exclusively
/// own the logical gauge, even when using an embedded-hal-bus shared-bus handle.
pub struct BQ27441<I2C, DELAY> {
    interface: I2cInterface<I2C, DELAY>,
    functions: Measurements,
}

macro_rules! scalar_reader {
    ($name:ident, $register:ty, $field:ident, $doc:literal) => {
        #[doc = $doc]
        pub fn $name(&mut self) -> Result<u16, DeviceError<I::Error>> {
            Ok(self.read_register::<$register>()?.$field())
        }
    };
}

impl<I: I2c, D: DelayNs> BQ27441<I, D> {
    /// Wait 250 ms for communication startup. No bus traffic or reset is issued.
    /// Call `identify` to validate the device and fixed chemistry.
    pub fn new(i2c: I, delay: D) -> Self {
        #[cfg(feature = "defmt")]
        defmt::debug!("BQ27441 startup");
        Self {
            interface: I2cInterface::new(i2c, delay),
            functions: Measurements,
        }
    }

    /// Return the owned bus handle and delay.
    pub fn release(self) -> (I, D) {
        self.interface.release()
    }

    /// Read a standard word or extended read-only mirror.
    ///
    /// ```compile_fail
    /// use bq27441::register::{Control, Register};
    /// fn readable<R: Register>() {}
    /// readable::<Control>();
    /// ```
    pub fn read_register<R: Register>(&mut self) -> Result<R, DeviceError<I::Error>> {
        self.interface
            .read_word(R::ADDRESS)
            .map(R::from)
            .map_err(DeviceError::from)
    }

    /// Query a response-producing Control subcommand.
    pub fn read_control(
        &mut self,
        command: ControlCommand,
    ) -> Result<Control, DeviceError<I::Error>> {
        if !command.has_response() {
            return Err(DeviceError::InvalidControlQuery.reported());
        }
        self.write_control(command)?;
        self.interface
            .read_word(CONTROL_ADDRESS)
            .map(Control::from)
            .map_err(DeviceError::from)
    }

    /// Read decoded CONTROL_STATUS without assuming a cached selection.
    pub fn control_status(&mut self) -> Result<ControlStatus, DeviceError<I::Error>> {
        Ok(ControlStatus::from(
            self.read_control(ControlCommand::Status)?.value(),
        ))
    }

    /// Verify device type, data-memory code and chemistry identifier together.
    pub fn identify(&mut self) -> Result<Variant, DeviceError<I::Error>> {
        let device_type = self.read_control(ControlCommand::DeviceType)?.value();
        let data_memory_code = self.read_control(ControlCommand::DataMemoryCode)?.value();
        let chemistry_identifier = self
            .read_control(ControlCommand::ChemistryIdentifier)?
            .value();
        match (device_type, data_memory_code, chemistry_identifier) {
            (0x0421, 0x0048, 0x0128) => Ok(Variant::G1A),
            (0x0421, 0x0058, 0x0312) => Ok(Variant::G1B),
            _ => Err(DeviceError::UnexpectedDevice {
                device_type,
                data_memory_code,
                chemistry_identifier,
            }
            .reported()),
        }
    }

    /// Issue an action, checking sealing and shutdown prerequisites.
    ///
    /// Low-level mode actions do not poll completion; use the dedicated methods
    /// for verified configuration entry/exit and sealing.
    pub fn send_command(&mut self, command: ControlCommand) -> Result<(), DeviceError<I::Error>> {
        if command.has_response() {
            return Err(DeviceError::InvalidControlAction.reported());
        }
        if command.requires_unsealed() {
            let status = self.control_status()?;
            if status.sealed() {
                return Err(DeviceError::Sealed.reported());
            }
            if command == ControlCommand::Shutdown && !status.shutdown_enabled() {
                return Err(DeviceError::ShutdownNotEnabled.reported());
            }
        }
        self.write_control(command)
    }

    fn write_control(&mut self, command: ControlCommand) -> Result<(), DeviceError<I::Error>> {
        self.interface
            .write_word(CONTROL_ADDRESS, u16::from(command))
            .map_err(DeviceError::from)
    }

    /// Send both fixed ROM key words consecutively, then verify SS clears.
    pub fn unseal(&mut self) -> Result<(), DeviceError<I::Error>> {
        for _ in 0..2 {
            self.interface
                .write_word(CONTROL_ADDRESS, UNSEAL_WORD)
                .map_err(DeviceError::from)?;
        }
        self.wait_for_sealed(false)
    }

    /// Seal and verify SS sets. Already-sealed devices remain sealed.
    pub fn seal(&mut self) -> Result<(), DeviceError<I::Error>> {
        self.write_control(ControlCommand::Seal)?;
        self.wait_for_sealed(true)
    }

    fn wait_for_sealed(&mut self, expected: bool) -> Result<(), DeviceError<I::Error>> {
        for _ in 0..POLL_ATTEMPTS {
            if self.control_status()?.sealed() == expected {
                return Ok(());
            }
        }
        Err(DeviceError::SealStateTimeout { expected }.reported())
    }

    fn require_unsealed(&mut self) -> Result<(), DeviceError<I::Error>> {
        if self.control_status()?.sealed() {
            return Err(DeviceError::Sealed.reported());
        }
        Ok(())
    }

    /// Enter CONFIG UPDATE after checking unsealed state and poll CFGUPMODE.
    pub fn enter_configuration_update(&mut self) -> Result<(), DeviceError<I::Error>> {
        self.send_command(ControlCommand::SetConfigurationUpdate)?;
        self.wait_for_configuration_update(true)
    }

    /// Soft-reset, resimulate and verify exit; this can automatically reseal.
    pub fn exit_configuration_update(&mut self) -> Result<(), DeviceError<I::Error>> {
        self.send_command(ControlCommand::SoftReset)?;
        self.wait_for_configuration_update(false)
    }

    fn wait_for_configuration_update(
        &mut self,
        expected: bool,
    ) -> Result<(), DeviceError<I::Error>> {
        for _ in 0..POLL_ATTEMPTS {
            if self.flags()?.configuration_update() == expected {
                return Ok(());
            }
        }
        Err(DeviceError::ConfigurationUpdateTimeout { expected }.reported())
    }

    /// Read all operating flags, including battery detection and POR.
    pub fn flags(&mut self) -> Result<Flags, DeviceError<I::Error>> {
        self.read_register()
    }

    /// Read the read-only OpConfig mirror.
    pub fn operation_configuration(
        &mut self,
    ) -> Result<OperationConfiguration, DeviceError<I::Error>> {
        self.read_register()
    }

    /// Read percentage and validity together. Do not ignore invalid status.
    pub fn state_of_health(&mut self) -> Result<StateOfHealth, DeviceError<I::Error>> {
        self.read_register()
    }

    /// Read temperature as exact integer centi-degrees Celsius.
    pub fn temperature_centicelsius(&mut self) -> Result<i32, DeviceError<I::Error>> {
        let value = self.read_register::<Temperature>()?;
        Ok(self.functions.temperature_centicelsius(value))
    }

    /// Read internal temperature even when external temperature is configured.
    pub fn internal_temperature_centicelsius(&mut self) -> Result<i32, DeviceError<I::Error>> {
        let value = self.read_register::<InternalTemperature>()?;
        Ok(self.functions.internal_temperature_centicelsius(value))
    }

    /// Supply external temperature (0.1 K), rejecting OpConfig.TEMPS=0.
    pub fn write_temperature(
        &mut self,
        temperature: Temperature,
    ) -> Result<(), DeviceError<I::Error>> {
        if !self.operation_configuration()?.external_temperature() {
            return Err(DeviceError::ExternalTemperatureDisabled.reported());
        }
        self.interface
            .write_word(Temperature::ADDRESS, temperature.into())
            .map_err(DeviceError::from)
    }

    /// Signed current in mA: negative means discharging.
    pub fn average_current_milliamps(&mut self) -> Result<i16, DeviceError<I::Error>> {
        let value = self.read_register::<AverageCurrent>()?;
        Ok(self.functions.average_current_milliamps(value))
    }

    /// Adaptive signed standby current in mA.
    pub fn standby_current_milliamps(&mut self) -> Result<i16, DeviceError<I::Error>> {
        let value = self.read_register::<StandbyCurrent>()?;
        Ok(self.functions.standby_current_milliamps(value))
    }

    /// Adaptive signed maximum load current in mA.
    pub fn maximum_load_current_milliamps(&mut self) -> Result<i16, DeviceError<I::Error>> {
        let value = self.read_register::<MaximumLoadCurrent>()?;
        Ok(self.functions.maximum_load_current_milliamps(value))
    }

    /// Signed average power in mW: negative means discharging.
    pub fn average_power_milliwatts(&mut self) -> Result<i16, DeviceError<I::Error>> {
        let value = self.read_register::<AveragePower>()?;
        Ok(self.functions.average_power_milliwatts(value))
    }

    scalar_reader!(
        voltage_millivolts,
        Voltage,
        millivolts,
        "Read cell voltage in mV."
    );
    scalar_reader!(
        nominal_available_capacity_milliamp_hours,
        NominalAvailableCapacity,
        milliamp_hours,
        "Read uncompensated remaining capacity in mAh."
    );
    scalar_reader!(
        full_available_capacity_milliamp_hours,
        FullAvailableCapacity,
        milliamp_hours,
        "Read uncompensated full capacity in mAh."
    );
    scalar_reader!(
        remaining_capacity_milliamp_hours,
        RemainingCapacity,
        milliamp_hours,
        "Read compensated remaining capacity in mAh."
    );
    scalar_reader!(
        full_charge_capacity_milliamp_hours,
        FullChargeCapacity,
        milliamp_hours,
        "Read compensated full capacity in mAh."
    );
    scalar_reader!(
        state_of_charge_percent,
        StateOfCharge,
        percent,
        "Read state of charge in percent."
    );
    scalar_reader!(
        remaining_capacity_unfiltered_milliamp_hours,
        RemainingCapacityUnfiltered,
        milliamp_hours,
        "Read unfiltered remaining capacity in mAh."
    );
    scalar_reader!(
        remaining_capacity_filtered_milliamp_hours,
        RemainingCapacityFiltered,
        milliamp_hours,
        "Read filtered remaining capacity in mAh."
    );
    scalar_reader!(
        full_charge_capacity_unfiltered_milliamp_hours,
        FullChargeCapacityUnfiltered,
        milliamp_hours,
        "Read unfiltered full capacity in mAh."
    );
    scalar_reader!(
        full_charge_capacity_filtered_milliamp_hours,
        FullChargeCapacityFiltered,
        milliamp_hours,
        "Read filtered full capacity in mAh."
    );
    scalar_reader!(
        state_of_charge_unfiltered_percent,
        StateOfChargeUnfiltered,
        percent,
        "Read unfiltered state of charge in percent."
    );
    scalar_reader!(
        design_capacity_milliamp_hours,
        DesignCapacity,
        milliamp_hours,
        "Read the design-capacity mirror in mAh."
    );

    fn check_data_block(
        subclass: DataSubclass,
        block: DataBlock,
    ) -> Result<(), DeviceError<I::Error>> {
        let last = match subclass {
            DataSubclass::ImpedanceTrackConfiguration => 2,
            DataSubclass::State => 1,
            _ => 0,
        };
        if block.number() > last {
            return Err(DeviceError::InvalidDataBlock.reported());
        }
        Ok(())
    }

    fn select_data_block(
        &mut self,
        subclass: DataSubclass,
        block: DataBlock,
    ) -> Result<(), DeviceError<I::Error>> {
        let control = BlockDataControl::from(0);
        let class = DataClass::from(u8::from(subclass));
        self.interface
            .write_byte(BLOCK_CONTROL_ADDRESS, control.into())
            .map_err(DeviceError::from)?;
        self.interface
            .write_byte(DATA_CLASS_ADDRESS, class.into())
            .map_err(DeviceError::from)?;
        self.interface
            .write_byte(DATA_BLOCK_ADDRESS, block.into())
            .map_err(DeviceError::from)?;
        self.interface.delay_ms(BLOCK_SELECTION_DELAY_MS);
        Ok(())
    }

    fn read_selected_block(&mut self) -> Result<BlockData, DeviceError<I::Error>> {
        let mut bytes = [0; 32];
        self.interface
            .read(BLOCK_DATA_ADDRESS, &mut bytes)
            .map_err(DeviceError::from)?;
        let block = BlockData::from(bytes);
        let mut received = [0];
        self.interface
            .read(BLOCK_CHECKSUM_ADDRESS, &mut received)
            .map_err(DeviceError::from)?;
        validation::validate_checksum(block, received[0].into()).map_err(
            |(expected, received)| DeviceError::ChecksumMismatch { expected, received }.reported(),
        )?;
        Ok(block)
    }

    /// Read and checksum-validate a documented data-memory page while unsealed.
    pub fn read_data_block(
        &mut self,
        subclass: DataSubclass,
        block: DataBlock,
    ) -> Result<BlockData, DeviceError<I::Error>> {
        Self::check_data_block(subclass, block)?;
        self.require_unsealed()?;
        self.select_data_block(subclass, block)?;
        self.read_selected_block()
    }

    /// Write, commit and verify a full data-memory page in CONFIG UPDATE.
    ///
    /// Low-level: preserve reserved/unrelated bytes and obey TRM parameter ranges.
    /// A failure can leave partial staging or an already committed page.
    pub fn write_data_block(
        &mut self,
        subclass: DataSubclass,
        block: DataBlock,
        data: BlockData,
    ) -> Result<(), DeviceError<I::Error>> {
        Self::check_data_block(subclass, block)?;
        if subclass == DataSubclass::SecurityCodes {
            return Err(DeviceError::ReadOnlyDataMemory.reported());
        }
        self.require_unsealed()?;
        if !self.flags()?.configuration_update() {
            return Err(DeviceError::ConfigurationUpdateRequired.reported());
        }
        self.write_selected_block(subclass, block, data)
    }

    fn write_selected_block(
        &mut self,
        subclass: DataSubclass,
        block: DataBlock,
        data: BlockData,
    ) -> Result<(), DeviceError<I::Error>> {
        self.select_data_block(subclass, block)?;
        for (offset, byte) in (0u8..32).zip(data.bytes) {
            self.interface
                .write_byte(BLOCK_DATA_ADDRESS + offset, byte.into())
                .map_err(DeviceError::from)?;
        }
        self.interface
            .write_byte(BLOCK_CHECKSUM_ADDRESS, validation::checksum(data).into())
            .map_err(DeviceError::from)?;
        self.interface.delay_ms(BLOCK_COMMIT_DELAY_MS);
        self.select_data_block(subclass, block)?;
        if self.read_selected_block()? != data {
            return Err(DeviceError::BlockVerificationFailed.reported());
        }
        Ok(())
    }

    /// Update the four essential cell parameters, preserving all other bytes.
    ///
    /// Unseals only if needed and restores sealing on success. On error, no
    /// cleanup is attempted: inspect/recover the device before resuming gauging.
    pub fn configure_battery(
        &mut self,
        configuration: BatteryConfiguration,
    ) -> Result<(), DeviceError<I::Error>> {
        if !(200..=8000).contains(&configuration.design_capacity_milliamp_hours) {
            return Err(DeviceError::InvalidDesignCapacity.reported());
        }
        if !(1..=32767).contains(&configuration.design_energy_milliwatt_hours) {
            return Err(DeviceError::InvalidDesignEnergy.reported());
        }
        if !(2500..=3700).contains(&configuration.terminate_voltage_millivolts) {
            return Err(DeviceError::InvalidTerminateVoltage.reported());
        }
        if !(1..=2000).contains(&configuration.taper_rate) {
            return Err(DeviceError::InvalidTaperRate.reported());
        }
        let was_sealed = self.control_status()?.sealed();
        if was_sealed {
            self.unseal()?;
        }
        self.enter_configuration_update()?;
        let block = DataBlock::from(0);
        let mut data = self.read_data_block(DataSubclass::State, block)?;
        for (offset, value) in [
            (10, configuration.design_capacity_milliamp_hours),
            (12, configuration.design_energy_milliwatt_hours),
            (16, configuration.terminate_voltage_millivolts),
            (27, configuration.taper_rate),
        ] {
            let [high, low] = value.to_be_bytes();
            data.bytes[offset] = high.into();
            data.bytes[offset + 1] = low.into();
        }
        self.write_selected_block(DataSubclass::State, block, data)?;
        self.exit_configuration_update()?;
        if was_sealed {
            self.seal()?;
        } else if self.control_status()?.sealed() {
            self.unseal()?;
        }
        #[cfg(feature = "defmt")]
        defmt::info!("BQ27441 battery configuration verified");
        Ok(())
    }
}
