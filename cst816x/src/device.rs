use embedded_hal::i2c::I2c;

use crate::{comproc, error::Cst816sError, registers::*};

/// CST816S touch controller, owning I2C and a private register-access helper.
///
/// Register definitions: CST816S_register_declaration.pdf, pages 1-5.
/// Construction does not reset, wake, configure, or communicate with the device.
///
/// ```
/// use cst816::{Cst816x, Cst816sError};
/// use embedded_hal::i2c::I2c;
///
/// fn read_x<I: I2c>(i2c: I) -> Result<(I, u16), Cst816sError<I::Error>> {
///     let mut device = Cst816x::new(i2c);
///     let x = device.read_x_position()?.coordinate().value();
///     Ok((device.release(), x))
/// }
/// ```
pub struct Cst816x<I2C> {
    addr: u8,
    i2c: I2C,
    register: Register,
}

impl<I: I2c> Cst816x<I> {
    /// Wrap a ready device at the default seven-bit address, 0x15, without I/O.
    pub fn new(i2c: I) -> Self {
        Self {
            i2c,
            register: Register,
            addr: comproc::I2C_ADDRESS,
        }
    }

    /// Wrap a ready device at a custom seven-bit address without I/O.
    ///
    /// Supply an unshifted address in 0x00-0x7F, not an eight-bit wire address.
    /// This selects the destination for transfers; it does not reprogram the IC.
    pub fn with_address(i2c: I, addr: u8) -> Self {
        Self {
            i2c,
            addr,
            register: Register,
        }
    }

    /// Return the owned bus without changing the device configuration.
    pub fn release(self) -> I {
        self.i2c
    }

    /// Read [`GestureIdentifier`].
    pub fn read_gesture_identifier(&mut self) -> Result<GestureIdentifier, Cst816sError<I::Error>> {
        self.register
            .read_gesture_identifier(&mut self.i2c, self.addr)
    }

    /// Read [`FingerNumber`].
    pub fn read_finger_number(&mut self) -> Result<FingerNumber, Cst816sError<I::Error>> {
        self.register.read_finger_number(&mut self.i2c, self.addr)
    }

    /// Read [`XPosition`].
    pub fn read_x_position(&mut self) -> Result<XPosition, Cst816sError<I::Error>> {
        self.register.read_x_position(&mut self.i2c, self.addr)
    }

    /// Read [`YPosition`].
    pub fn read_y_position(&mut self) -> Result<YPosition, Cst816sError<I::Error>> {
        self.register.read_y_position(&mut self.i2c, self.addr)
    }

    /// Read [`ChipIdentifier`].
    pub fn read_chip_identifier(&mut self) -> Result<ChipIdentifier, Cst816sError<I::Error>> {
        self.register.read_chip_identifier(&mut self.i2c, self.addr)
    }

    /// Read [`ProjectIdentifier`].
    pub fn read_project_identifier(&mut self) -> Result<ProjectIdentifier, Cst816sError<I::Error>> {
        self.register
            .read_project_identifier(&mut self.i2c, self.addr)
    }

    /// Read [`FirmwareVersion`].
    pub fn read_firmware_version(&mut self) -> Result<FirmwareVersion, Cst816sError<I::Error>> {
        self.register
            .read_firmware_version(&mut self.i2c, self.addr)
    }

    /// Read [`Bpc0`].
    pub fn read_bpc0(&mut self) -> Result<Bpc0, Cst816sError<I::Error>> {
        self.register.read_bpc0(&mut self.i2c, self.addr)
    }

    /// Read [`Bpc1`].
    pub fn read_bpc1(&mut self) -> Result<Bpc1, Cst816sError<I::Error>> {
        self.register.read_bpc1(&mut self.i2c, self.addr)
    }

    /// Read [`MotionMask`].
    pub fn read_motion_mask(&mut self) -> Result<MotionMask, Cst816sError<I::Error>> {
        self.register.read_motion_mask(&mut self.i2c, self.addr)
    }

    /// Read [`InterruptPulseWidth`].
    pub fn read_interrupt_pulse_width(
        &mut self,
    ) -> Result<InterruptPulseWidth, Cst816sError<I::Error>> {
        self.register
            .read_interrupt_pulse_width(&mut self.i2c, self.addr)
    }

    /// Read [`NormalScanPeriod`].
    pub fn read_normal_scan_period(&mut self) -> Result<NormalScanPeriod, Cst816sError<I::Error>> {
        self.register
            .read_normal_scan_period(&mut self.i2c, self.addr)
    }

    /// Read [`MotionSlidingAngle`].
    pub fn read_motion_sliding_angle(
        &mut self,
    ) -> Result<MotionSlidingAngle, Cst816sError<I::Error>> {
        self.register
            .read_motion_sliding_angle(&mut self.i2c, self.addr)
    }

    /// Read [`LowPowerScanRaw1`].
    pub fn read_low_power_scan_raw1(&mut self) -> Result<LowPowerScanRaw1, Cst816sError<I::Error>> {
        self.register
            .read_low_power_scan_raw1(&mut self.i2c, self.addr)
    }

    /// Read [`LowPowerScanRaw2`].
    pub fn read_low_power_scan_raw2(&mut self) -> Result<LowPowerScanRaw2, Cst816sError<I::Error>> {
        self.register
            .read_low_power_scan_raw2(&mut self.i2c, self.addr)
    }

    /// Read [`LowPowerAutoWakeTime`].
    pub fn read_low_power_auto_wake_time(
        &mut self,
    ) -> Result<LowPowerAutoWakeTime, Cst816sError<I::Error>> {
        self.register
            .read_low_power_auto_wake_time(&mut self.i2c, self.addr)
    }

    /// Read [`LowPowerScanThreshold`].
    pub fn read_low_power_scan_threshold(
        &mut self,
    ) -> Result<LowPowerScanThreshold, Cst816sError<I::Error>> {
        self.register
            .read_low_power_scan_threshold(&mut self.i2c, self.addr)
    }

    /// Read [`LowPowerScanWindow`].
    pub fn read_low_power_scan_window(
        &mut self,
    ) -> Result<LowPowerScanWindow, Cst816sError<I::Error>> {
        self.register
            .read_low_power_scan_window(&mut self.i2c, self.addr)
    }

    /// Read [`LowPowerScanFrequency`].
    pub fn read_low_power_scan_frequency(
        &mut self,
    ) -> Result<LowPowerScanFrequency, Cst816sError<I::Error>> {
        self.register
            .read_low_power_scan_frequency(&mut self.i2c, self.addr)
    }

    /// Read [`LowPowerScanCurrent`].
    pub fn read_low_power_scan_current(
        &mut self,
    ) -> Result<LowPowerScanCurrent, Cst816sError<I::Error>> {
        self.register
            .read_low_power_scan_current(&mut self.i2c, self.addr)
    }

    /// Read [`AutoSleepTime`].
    pub fn read_auto_sleep_time(&mut self) -> Result<AutoSleepTime, Cst816sError<I::Error>> {
        self.register.read_auto_sleep_time(&mut self.i2c, self.addr)
    }

    /// Read [`InterruptControl`].
    pub fn read_interrupt_control(&mut self) -> Result<InterruptControl, Cst816sError<I::Error>> {
        self.register
            .read_interrupt_control(&mut self.i2c, self.addr)
    }

    /// Read [`AutoReset`].
    pub fn read_auto_reset(&mut self) -> Result<AutoReset, Cst816sError<I::Error>> {
        self.register.read_auto_reset(&mut self.i2c, self.addr)
    }

    /// Read [`LongPressTime`].
    pub fn read_long_press_time(&mut self) -> Result<LongPressTime, Cst816sError<I::Error>> {
        self.register.read_long_press_time(&mut self.i2c, self.addr)
    }

    /// Read [`IoControl`].
    pub fn read_io_control(&mut self) -> Result<IoControl, Cst816sError<I::Error>> {
        self.register.read_io_control(&mut self.i2c, self.addr)
    }

    /// Read [`AutoSleepDisable`].
    pub fn read_auto_sleep_disable(&mut self) -> Result<AutoSleepDisable, Cst816sError<I::Error>> {
        self.register
            .read_auto_sleep_disable(&mut self.i2c, self.addr)
    }

    /// Write [`Bpc0`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_bpc0(&mut self, value: Bpc0) -> Result<(), Cst816sError<I::Error>> {
        self.register.write_bpc0(&mut self.i2c, self.addr, value)
    }

    /// Write [`Bpc1`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_bpc1(&mut self, value: Bpc1) -> Result<(), Cst816sError<I::Error>> {
        self.register.write_bpc1(&mut self.i2c, self.addr, value)
    }

    /// Write [`MotionMask`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_motion_mask(&mut self, value: MotionMask) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_motion_mask(&mut self.i2c, self.addr, value)
    }

    /// Write [`InterruptPulseWidth`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_interrupt_pulse_width(
        &mut self,
        value: InterruptPulseWidth,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_interrupt_pulse_width(&mut self.i2c, self.addr, value)
    }

    /// Write [`NormalScanPeriod`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_normal_scan_period(
        &mut self,
        value: NormalScanPeriod,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_normal_scan_period(&mut self.i2c, self.addr, value)
    }

    /// Write [`MotionSlidingAngle`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_motion_sliding_angle(
        &mut self,
        value: MotionSlidingAngle,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_motion_sliding_angle(&mut self.i2c, self.addr, value)
    }

    /// Write [`LowPowerScanRaw1`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_low_power_scan_raw1(
        &mut self,
        value: LowPowerScanRaw1,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_low_power_scan_raw1(&mut self.i2c, self.addr, value)
    }

    /// Write [`LowPowerScanRaw2`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_low_power_scan_raw2(
        &mut self,
        value: LowPowerScanRaw2,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_low_power_scan_raw2(&mut self.i2c, self.addr, value)
    }

    /// Write [`LowPowerAutoWakeTime`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_low_power_auto_wake_time(
        &mut self,
        value: LowPowerAutoWakeTime,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_low_power_auto_wake_time(&mut self.i2c, self.addr, value)
    }

    /// Write [`LowPowerScanThreshold`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_low_power_scan_threshold(
        &mut self,
        value: LowPowerScanThreshold,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_low_power_scan_threshold(&mut self.i2c, self.addr, value)
    }

    /// Write [`LowPowerScanWindow`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_low_power_scan_window(
        &mut self,
        value: LowPowerScanWindow,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_low_power_scan_window(&mut self.i2c, self.addr, value)
    }

    /// Write [`LowPowerScanFrequency`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_low_power_scan_frequency(
        &mut self,
        value: LowPowerScanFrequency,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_low_power_scan_frequency(&mut self.i2c, self.addr, value)
    }

    /// Write [`LowPowerScanCurrent`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_low_power_scan_current(
        &mut self,
        value: LowPowerScanCurrent,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_low_power_scan_current(&mut self.i2c, self.addr, value)
    }

    /// Write [`AutoSleepTime`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_auto_sleep_time(
        &mut self,
        value: AutoSleepTime,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_auto_sleep_time(&mut self.i2c, self.addr, value)
    }

    /// Write [`InterruptControl`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_interrupt_control(
        &mut self,
        value: InterruptControl,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_interrupt_control(&mut self.i2c, self.addr, value)
    }

    /// Write [`AutoReset`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_auto_reset(&mut self, value: AutoReset) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_auto_reset(&mut self.i2c, self.addr, value)
    }

    /// Write [`LongPressTime`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_long_press_time(
        &mut self,
        value: LongPressTime,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_long_press_time(&mut self.i2c, self.addr, value)
    }

    /// Write [`IoControl`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_io_control(&mut self, value: IoControl) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_io_control(&mut self.i2c, self.addr, value)
    }

    /// Write [`AutoSleepDisable`].
    /// See the device README for preservation and partial-failure semantics.
    pub fn write_auto_sleep_disable(
        &mut self,
        value: AutoSleepDisable,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_auto_sleep_disable(&mut self.i2c, self.addr, value)
    }
}
