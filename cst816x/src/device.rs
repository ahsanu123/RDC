use embedded_hal::i2c::I2c;

use crate::{
    comproc,
    error::{Cst816sAddressError, Cst816sError},
    function::{TimingFunction, TouchFunction},
    register::*,
};

/// Unscaled 12-bit controller coordinates; no screen geometry is assumed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TouchPosition {
    pub x: u16,
    pub y: u16,
}

/// Decoded touch window, not a guaranteed hardware-latched snapshot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TouchReading {
    /// Gesture is retained even when there is no current finger.
    pub gesture: Gesture,
    /// `None` means FingerNum = 0; stale coordinate bytes are not reported.
    pub position: Option<TouchPosition>,
}

/// CST816S touch controller, owning I2C and a private register-access helper.
///
/// Register definitions: CST816S_register_declaration.pdf, pages 1-5.
/// Construction does not reset, wake, configure, or communicate with the device.
/// See README [Shared bus](../index.html#shared-bus) and
/// [Partial failures](../index.html#partial-failures) for operation-wide semantics.
///
/// ```
/// use cst816::{Cst816s, Cst816sError};
/// use embedded_hal::i2c::I2c;
///
/// fn read_x<I: I2c>(i2c: I) -> Result<(I, u16), Cst816sError<I::Error>> {
///     let mut device = Cst816s::new(i2c);
///     let x = device.read_x_position()?.coordinate().value();
///     Ok((device.release(), x))
/// }
/// ```
pub struct Cst816s<I2C> {
    addr: u8,
    i2c: I2C,
    register: Register,
    touch_function: TouchFunction,
    timing_function: TimingFunction,
}

/// Compatibility alias; the register definitions support CST816S only.
#[deprecated(
    since = "0.1.0",
    note = "use Cst816s; other CST816 variants are not verified"
)]
pub type Cst816x<I> = Cst816s<I>;

impl<I: I2c> Cst816s<I> {
    /// Wrap a ready device at the default seven-bit address, 0x15, without I/O.
    pub fn new(i2c: I) -> Self {
        Self {
            i2c,
            register: Register,
            touch_function: TouchFunction,
            timing_function: TimingFunction,
            addr: comproc::I2C_ADDRESS,
        }
    }

    /// Wrap a ready device at a custom seven-bit address without I/O.
    ///
    /// Supply an unshifted address in 0x00-0x7F, not an eight-bit wire address.
    /// This selects the destination for transfers; it does not reprogram the IC.
    /// Invalid addresses return [`Cst816sAddressError`] containing the owned bus.
    pub fn with_address(i2c: I, addr: u8) -> Result<Self, Cst816sAddressError<I>> {
        if addr > 0x7F {
            return Err(Cst816sAddressError { address: addr, i2c });
        }
        Ok(Self {
            i2c,
            addr,
            register: Register,
            touch_function: TouchFunction,
            timing_function: TimingFunction,
        })
    }

    /// Return the owned bus without changing the device configuration.
    pub fn release(self) -> I {
        self.i2c
    }

    /// Read 0x01-0x06 with one pointer write and six-byte repeated-start read.
    ///
    /// Invalid gesture/count bytes are rejected without masking them. No finger
    /// means `position: None`, even if coordinate bytes contain an old position.
    /// Sequential framing is independently corroborated, but sample latching is
    /// undocumented: see README [Touch consistency](../index.html#touch-consistency)
    /// and [Startup, power and interrupts](../index.html#startup-power-and-interrupts).
    pub fn read_touch(&mut self) -> Result<TouchReading, Cst816sError<I::Error>> {
        let (gesture, fingers, x, y) = self.register.read_touch(&mut self.i2c, self.addr)?;
        Ok(self.touch_function.process(gesture, fingers, x, y))
    }

    /// Read interrupt pulse width and convert 0.1 ms units to microseconds.
    /// This is a raw conversion, not validation of the hardware's current setting.
    pub fn read_interrupt_pulse_width_microseconds(
        &mut self,
    ) -> Result<u32, Cst816sError<I::Error>> {
        let value = self.read_interrupt_pulse_width()?;
        Ok(self.timing_function.interrupt_pulse_microseconds(value))
    }

    /// Read normal scan period and convert 10 ms units to milliseconds.
    /// This is a raw conversion; it does not predict the undocumented timing coupling.
    pub fn read_normal_scan_period_milliseconds(&mut self) -> Result<u16, Cst816sError<I::Error>> {
        let value = self.read_normal_scan_period()?;
        Ok(self.timing_function.normal_scan_milliseconds(value))
    }

    /// Read low-power recalibration period and convert minutes to seconds.
    /// This is a raw conversion, not validation of the hardware's current setting.
    pub fn read_low_power_auto_wake_time_seconds(&mut self) -> Result<u16, Cst816sError<I::Error>> {
        let value = self.read_low_power_auto_wake_time()?;
        Ok(self.timing_function.auto_wake_seconds(value))
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
    /// Uses one two-byte sequential read, not a guaranteed latched sample.
    /// See README [Touch consistency](../index.html#touch-consistency).
    pub fn read_x_position(&mut self) -> Result<XPosition, Cst816sError<I::Error>> {
        self.register.read_x_position(&mut self.i2c, self.addr)
    }

    /// Read [`YPosition`].
    /// Uses one two-byte sequential read, not a guaranteed latched sample.
    /// See README [Touch consistency](../index.html#touch-consistency).
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
    /// Two separate reads; see README [Partial failures](../index.html#partial-failures).
    pub fn read_bpc0(&mut self) -> Result<Bpc0, Cst816sError<I::Error>> {
        self.register.read_bpc0(&mut self.i2c, self.addr)
    }

    /// Read [`Bpc1`].
    /// Two separate reads; see README [Partial failures](../index.html#partial-failures).
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
    /// Two separate reads; see README [Partial failures](../index.html#partial-failures).
    pub fn read_low_power_scan_raw1(&mut self) -> Result<LowPowerScanRaw1, Cst816sError<I::Error>> {
        self.register
            .read_low_power_scan_raw1(&mut self.i2c, self.addr)
    }

    /// Read [`LowPowerScanRaw2`].
    /// Two separate reads; see README [Partial failures](../index.html#partial-failures).
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

    /// Write [`MotionMask`].
    /// Reads first to preserve unspecified bits, replacing every documented field.
    /// See README [Shared bus](../index.html#shared-bus) and
    /// [Partial failures](../index.html#partial-failures).
    pub fn write_motion_mask(&mut self, value: MotionMask) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_motion_mask(&mut self.i2c, self.addr, value)
    }

    /// Write [`InterruptPulseWidth`].
    /// Rejects raw values outside 1..=200 before I2C.
    /// See README [Partial failures](../index.html#partial-failures).
    pub fn write_interrupt_pulse_width(
        &mut self,
        value: InterruptPulseWidth,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_interrupt_pulse_width(&mut self.i2c, self.addr, value)
    }

    /// Write [`NormalScanPeriod`].
    /// Rejects raw values outside 1..=30 before I2C.
    /// See README [Partial failures](../index.html#partial-failures).
    pub fn write_normal_scan_period(
        &mut self,
        value: NormalScanPeriod,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_normal_scan_period(&mut self.i2c, self.addr, value)
    }

    /// Write [`MotionSlidingAngle`].
    /// No angle-unit conversion is inferred from the incomplete formula.
    /// See README [Partial failures](../index.html#partial-failures).
    pub fn write_motion_sliding_angle(
        &mut self,
        value: MotionSlidingAngle,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_motion_sliding_angle(&mut self.i2c, self.addr, value)
    }

    /// Write [`LowPowerAutoWakeTime`].
    /// Rejects raw values outside 1..=5 before I2C.
    /// See README [Partial failures](../index.html#partial-failures).
    pub fn write_low_power_auto_wake_time(
        &mut self,
        value: LowPowerAutoWakeTime,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_low_power_auto_wake_time(&mut self.i2c, self.addr, value)
    }

    /// Write [`LowPowerScanThreshold`].
    /// Rejects zero before I2C.
    /// See README [Partial failures](../index.html#partial-failures).
    pub fn write_low_power_scan_threshold(
        &mut self,
        value: LowPowerScanThreshold,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_low_power_scan_threshold(&mut self.i2c, self.addr, value)
    }

    /// Write [`LowPowerScanWindow`].
    /// Rejects raw values outside 0..=3 before I2C.
    /// See README [Partial failures](../index.html#partial-failures).
    pub fn write_low_power_scan_window(
        &mut self,
        value: LowPowerScanWindow,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_low_power_scan_window(&mut self.i2c, self.addr, value)
    }

    /// Write [`LowPowerScanFrequency`].
    /// Rejects zero before I2C; the physical frequency unit is unspecified.
    /// See README [Partial failures](../index.html#partial-failures).
    pub fn write_low_power_scan_frequency(
        &mut self,
        value: LowPowerScanFrequency,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_low_power_scan_frequency(&mut self.i2c, self.addr, value)
    }

    /// Write [`LowPowerScanCurrent`].
    /// Rejects zero before I2C; the physical current unit is unspecified.
    /// See README [Partial failures](../index.html#partial-failures).
    pub fn write_low_power_scan_current(
        &mut self,
        value: LowPowerScanCurrent,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_low_power_scan_current(&mut self.i2c, self.addr, value)
    }

    /// Write [`AutoSleepTime`].
    /// See README [Shared bus](../index.html#shared-bus) and
    /// [Partial failures](../index.html#partial-failures).
    pub fn write_auto_sleep_time(
        &mut self,
        value: AutoSleepTime,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_auto_sleep_time(&mut self.i2c, self.addr, value)
    }

    /// Write [`InterruptControl`].
    /// Bit 4 / bit 0 assignments remain provisional because the manufacturer's
    /// drawing conflicts with its prose. See README [IRQ mapping](../index.html#irq-mapping).
    /// Reads first to preserve unspecified bits, replacing every documented field.
    /// See README [Shared bus](../index.html#shared-bus) and
    /// [Partial failures](../index.html#partial-failures).
    pub fn write_interrupt_control(
        &mut self,
        value: InterruptControl,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_interrupt_control(&mut self.i2c, self.addr, value)
    }

    /// Write [`AutoReset`].
    /// See README [Startup, power and interrupts](../index.html#startup-power-and-interrupts)
    /// and [Partial failures](../index.html#partial-failures).
    pub fn write_auto_reset(&mut self, value: AutoReset) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_auto_reset(&mut self.i2c, self.addr, value)
    }

    /// Write [`LongPressTime`].
    /// See README [Startup, power and interrupts](../index.html#startup-power-and-interrupts)
    /// and [Partial failures](../index.html#partial-failures).
    pub fn write_long_press_time(
        &mut self,
        value: LongPressTime,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_long_press_time(&mut self.i2c, self.addr, value)
    }

    /// Write [`IoControl`].
    /// Changes device-wide electrical/IRQ-reset behavior; configure compatible
    /// host levels first. See README [Startup, power and interrupts](../index.html#startup-power-and-interrupts).
    /// Reads first to preserve unspecified bits, replacing every documented field.
    /// See README [Shared bus](../index.html#shared-bus) and
    /// [Partial failures](../index.html#partial-failures).
    pub fn write_io_control(&mut self, value: IoControl) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_io_control(&mut self.i2c, self.addr, value)
    }

    /// Write [`AutoSleepDisable`].
    /// See README [Shared bus](../index.html#shared-bus) and
    /// [Partial failures](../index.html#partial-failures).
    pub fn write_auto_sleep_disable(
        &mut self,
        value: AutoSleepDisable,
    ) -> Result<(), Cst816sError<I::Error>> {
        self.register
            .write_auto_sleep_disable(&mut self.i2c, self.addr, value)
    }
}
