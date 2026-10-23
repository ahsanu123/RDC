use bilge::prelude::{u5, u7};
use embedded_hal::i2c::I2c;

use crate::{Error, comproc, function::Functions, register::*};

/// Fixed seven-bit I2C address.
pub const ADDR: u8 = 0x36;
const STATUS_FLAGS: u16 = 0x3f00;

/// MAX17048 one-cell ModelGauge fuel gauge (datasheet Rev. 7).
///
/// Owns the bus or an embedded-hal-compatible shared-bus proxy. No cached
/// register state is used.
pub struct MAX17048<I2C> {
    i2c: I2C,
    functions: Functions,
}

impl<I2C> MAX17048<I2C> {
    /// Construct without bus traffic or implicit resets.
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
            functions: Functions,
        }
    }

    /// Return the owned bus or bus proxy without changing the device.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<I2C: I2c> MAX17048<I2C> {
    /// Read a typed word in one repeated-START transaction.
    pub fn read_register<R: ReadableRegister>(&mut self) -> Result<R, Error<I2C::Error>> {
        comproc::read_word(&mut self.i2c, R::ADDRESS)
            .map(R::from)
            .map_err(Error::I2C)
    }

    fn update<R: ReadableRegister + Into<u16>>(
        &mut self,
        update: impl FnOnce(&mut R),
    ) -> Result<(), Error<I2C::Error>> {
        let mut value = self.read_register::<R>()?;
        update(&mut value);
        comproc::write_word(&mut self.i2c, R::ADDRESS, value.into()).map_err(Error::I2C)
    }

    /// Measured single-cell voltage in volts.
    pub fn voltage(&mut self) -> Result<f32, Error<I2C::Error>> {
        let value = self.read_register()?;
        Ok(self.functions.voltage(value))
    }

    /// ModelGauge state of charge in percent, including the fractional byte.
    ///
    /// Values are not clamped to 100%.
    pub fn soc(&mut self) -> Result<f32, Error<I2C::Error>> {
        let value = self.read_register()?;
        Ok(self.functions.state_of_charge(value))
    }

    /// Signed charge/discharge rate in percent per hour.
    pub fn charge_rate(&mut self) -> Result<f32, Error<I2C::Error>> {
        let value = self.read_register()?;
        Ok(self.functions.charge_rate(value))
    }

    /// IC production version, without rejecting future silicon revisions.
    pub fn version(&mut self) -> Result<u16, Error<I2C::Error>> {
        Ok(self.read_register::<Version>()?.production_version())
    }

    /// Restart SOC estimation. Only use with a fully relaxed battery.
    ///
    /// Preserves sleep permission; does not wait for new measurements.
    pub fn quick_start(&mut self) -> Result<(), Error<I2C::Error>> {
        self.update::<Mode>(|value| value.set_quick_start(true))
    }

    /// Send the power-on reset command.
    ///
    /// The IC does not ACK the final byte: the expected NACK is returned as
    /// `Error::I2C`, not silently treated as success. The caller must wait and
    /// verify reset state before reconfiguring; see README.md.
    pub fn reset(&mut self) -> Result<(), Error<I2C::Error>> {
        let command = Command::from(Command::POWER_ON_RESET);
        comproc::write_word(&mut self.i2c, Command::ADDRESS, command.into()).map_err(Error::I2C)
    }

    /// Read hibernate thresholds as raw typed fields.
    pub fn get_hibernate_threshold(&mut self) -> Result<HibernateThreshold, Error<I2C::Error>> {
        self.read_register()
    }

    /// Replace both thresholds. `0x0000` disables and `0xffff` forces hibernate.
    pub fn set_hibernate_threshold(
        &mut self,
        threshold: HibernateThreshold,
    ) -> Result<(), Error<I2C::Error>> {
        self.update::<HibernateThreshold>(|value| *value = threshold)
    }

    /// Hibernate-entry threshold in percent per hour.
    pub fn hibernate_threshold(&mut self) -> Result<f32, Error<I2C::Error>> {
        let value = self.read_register()?;
        Ok(self.functions.hibernate_threshold(value))
    }

    /// Hibernate-exit activity threshold in volts.
    pub fn activity_threshold(&mut self) -> Result<f32, Error<I2C::Error>> {
        let value = self.read_register()?;
        Ok(self.functions.activity_threshold(value))
    }

    /// Replace only the hibernate-entry count (0.208 %/hour per count).
    pub fn set_hibernate_entry_threshold(&mut self, count: u8) -> Result<(), Error<I2C::Error>> {
        self.update::<HibernateThreshold>(|value| value.set_hibernate_threshold(count))
    }

    /// Replace only the hibernate-exit count (1.25 mV per count).
    pub fn set_activity_threshold(&mut self, count: u8) -> Result<(), Error<I2C::Error>> {
        self.update::<HibernateThreshold>(|value| value.set_activity_threshold(count))
    }

    /// Report the current MODE.HibStat bit.
    pub fn is_hibernating(&mut self) -> Result<bool, Error<I2C::Error>> {
        Ok(self.read_register::<Mode>()?.hibernating())
    }

    /// Set sleep permission without replaying a pending QuickStart command.
    pub fn enable_sleep(&mut self, enabled: bool) -> Result<(), Error<I2C::Error>> {
        self.update::<Mode>(|value| {
            value.set_quick_start(false);
            value.set_sleep_enabled(enabled);
        })
    }

    /// Enter or leave software sleep.
    ///
    /// Entering first enables MODE.EnSleep. If the later CONFIG operation
    /// fails, sleep permission remains enabled; no rollback is attempted.
    pub fn sleep(&mut self, sleeping: bool) -> Result<(), Error<I2C::Error>> {
        if sleeping {
            self.enable_sleep(true)?;
        }
        self.update_configuration(|value| value.set_sleeping(sleeping))
    }

    /// Read the complete CONFIG word.
    pub fn get_config(&mut self) -> Result<Configuration, Error<I2C::Error>> {
        self.read_register()
    }

    /// Replace CONFIG policy fields, leaving ALRT unchanged.
    ///
    /// Use `clear_alert` to acknowledge ALRT explicitly.
    pub fn set_config(&mut self, config: Configuration) -> Result<(), Error<I2C::Error>> {
        self.update_configuration(|value| *value = config)
    }

    fn update_configuration(
        &mut self,
        update: impl FnOnce(&mut Configuration),
    ) -> Result<(), Error<I2C::Error>> {
        self.update::<Configuration>(|value| {
            update(value);
            // Write-one preserves the write-zero-clear latch, including new alerts.
            value.set_alert(true);
        })
    }

    /// Update battery compensation without changing alert or sleep settings.
    pub fn set_compensation(&mut self, compensation: u8) -> Result<(), Error<I2C::Error>> {
        self.update_configuration(|value| value.set_compensation(compensation))
    }

    /// Configure falling-edge low-SOC alert, from 1 through 32 percent.
    pub fn set_empty_alert_threshold(&mut self, percent: u8) -> Result<(), Error<I2C::Error>> {
        if !(1..=32).contains(&percent) {
            return Err(Error::InvalidInputData);
        }
        self.update_configuration(|value| {
            value.set_empty_alert_threshold(u5::new(32 - percent));
        })
    }

    /// Read the low-SOC alert threshold in percent.
    pub fn empty_alert_threshold(&mut self) -> Result<u8, Error<I2C::Error>> {
        let value = self.read_register()?;
        Ok(self.functions.empty_alert_threshold(value))
    }

    /// Enable or disable alerts on a 1% SOC change.
    pub fn enable_state_of_charge_change_alert(
        &mut self,
        enabled: bool,
    ) -> Result<(), Error<I2C::Error>> {
        self.update_configuration(|value| value.set_state_of_charge_change_alert(enabled))
    }

    /// Read the CONFIG.ALRT latch.
    pub fn is_active_alert(&mut self) -> Result<bool, Error<I2C::Error>> {
        Ok(self.get_config()?.alert())
    }

    /// Clear CONFIG.ALRT to release the ALRT pin, preserving all policy fields.
    pub fn clear_alert(&mut self) -> Result<(), Error<I2C::Error>> {
        self.update::<Configuration>(|value| value.set_alert(false))
    }

    /// Read the complete STATUS word without acknowledging any flags.
    pub fn get_status(&mut self) -> Result<Status, Error<I2C::Error>> {
        self.read_register()
    }

    /// Set STATUS.EnVR only; flag fields in the argument are ignored.
    ///
    /// Use `clear_status_flags` to explicitly acknowledge selected events.
    pub fn set_status(&mut self, status: Status) -> Result<(), Error<I2C::Error>> {
        self.update::<Status>(|value| {
            let preserved = u16::from(*value) | STATUS_FLAGS;
            *value = Status::from(preserved);
            value.set_voltage_reset_alert_enabled(status.voltage_reset_alert_enabled());
        })
    }

    /// Clear selected STATUS flags (true fields request clearing).
    ///
    /// EnVR and reserved fields in `flags` are ignored. Does not clear ALRT.
    pub fn clear_status_flags(&mut self, flags: Status) -> Result<(), Error<I2C::Error>> {
        self.update::<Status>(|value| {
            *value = Status::from(
                (u16::from(*value) | STATUS_FLAGS) & !(u16::from(flags) & STATUS_FLAGS),
            );
        })
    }

    /// Set voltage-alert limits in mV, exact multiples of 20, 0..=5100.
    ///
    /// Rejects reversed limits instead of silently clamping or rounding.
    pub fn set_voltage_alert(
        &mut self,
        minimum_mv: u16,
        maximum_mv: u16,
    ) -> Result<(), Error<I2C::Error>> {
        if minimum_mv > maximum_mv
            || maximum_mv > 5100
            || !minimum_mv.is_multiple_of(20)
            || !maximum_mv.is_multiple_of(20)
        {
            return Err(Error::InvalidInputData);
        }
        self.update::<VoltageAlert>(|value| {
            value.set_minimum((minimum_mv / 20) as u8);
            value.set_maximum((maximum_mv / 20) as u8);
        })
    }

    /// Voltage-alert minimum and maximum in volts.
    pub fn voltage_alert(&mut self) -> Result<(f32, f32), Error<I2C::Error>> {
        let value = self.read_register()?;
        Ok(self.functions.voltage_alert(value))
    }

    /// Set reset threshold in mV, an exact multiple of 40, 0..=5080.
    pub fn set_reset_voltage(&mut self, millivolts: u16) -> Result<(), Error<I2C::Error>> {
        if millivolts > 5080 || !millivolts.is_multiple_of(40) {
            return Err(Error::InvalidInputData);
        }
        self.update::<ResetVoltage>(|value| {
            value.set_threshold(u7::new((millivolts / 40) as u8));
        })
    }

    /// Reset threshold in volts.
    pub fn reset_voltage(&mut self) -> Result<f32, Error<I2C::Error>> {
        let value = self.read_register()?;
        Ok(self.functions.reset_voltage(value))
    }

    /// Enable or disable the fast analog reset comparator during hibernate.
    ///
    /// Preserves the threshold and factory ID.
    pub fn enable_reset_comparator(&mut self, enabled: bool) -> Result<(), Error<I2C::Error>> {
        self.update::<ResetVoltage>(|value| {
            value.set_comparator(if enabled {
                ResetComparator::Enabled
            } else {
                ResetComparator::Disabled
            });
        })
    }

    /// Read the factory-programmed eight-bit identifier.
    pub fn chip_id(&mut self) -> Result<u8, Error<I2C::Error>> {
        Ok(self.read_register::<ResetVoltage>()?.identifier())
    }

    /// Transfer 32 manufacturer-supplied model words and always attempt relock.
    ///
    /// Does not generate, qualify, or verify a model. Stops at the first
    /// unlock/data failure and attempts both lock writes, reporting all errors.
    /// A partial table is not rolled back. See README.md.
    pub fn write_model_table(
        &mut self,
        table: &[ModelTableWord; ModelTableWord::COUNT],
    ) -> Result<(), Error<I2C::Error>> {
        let operation = (|| {
            comproc::write_model_access(&mut self.i2c, ModelAccess::from(ModelAccess::UNLOCK_KEY))?;
            for (index, word) in table.iter().enumerate() {
                // Separate word transactions avoid the documented auto-increment limit.
                comproc::write_word(
                    &mut self.i2c,
                    ModelTableWord::START_ADDRESS + 2 * index as u8,
                    (*word).into(),
                )?;
            }
            Ok(())
        })()
        .err();
        let relock_low = comproc::write_byte(
            &mut self.i2c,
            ModelAccess::LOW_ADDRESS,
            ModelAccess::LOCK_BYTE,
        )
        .err();
        let relock_high = comproc::write_byte(
            &mut self.i2c,
            ModelAccess::HIGH_ADDRESS,
            ModelAccess::LOCK_BYTE,
        )
        .err();
        if operation.is_some() || relock_low.is_some() || relock_high.is_some() {
            return Err(Error::ModelTable {
                operation,
                relock_low,
                relock_high,
            });
        }
        Ok(())
    }
}
