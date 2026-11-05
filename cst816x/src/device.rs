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

macro_rules! read_register {
    ($method:ident, $register:ident) => {
        #[doc = concat!("Read [`", stringify!($register), "`].")]
        pub fn $method(&mut self) -> Result<$register, Cst816sError<I::Error>> {
            self.register.$method(&mut self.i2c, self.addr)
        }
    };
}

macro_rules! write_register {
    ($method:ident, $register:ident) => {
        #[doc = concat!(
                    "Write [`", stringify!($register),
                    "`]. See the device README for preservation and partial-failure semantics."
                )]
        pub fn $method(&mut self, value: $register) -> Result<(), Cst816sError<I::Error>> {
            self.register.$method(&mut self.i2c, self.addr, value)
        }
    };
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

    read_register!(read_gesture_identifier, GestureIdentifier);
    read_register!(read_finger_number, FingerNumber);
    read_register!(read_x_position, XPosition);
    read_register!(read_y_position, YPosition);
    read_register!(read_chip_identifier, ChipIdentifier);
    read_register!(read_project_identifier, ProjectIdentifier);
    read_register!(read_firmware_version, FirmwareVersion);
    read_register!(read_bpc0, Bpc0);
    read_register!(read_bpc1, Bpc1);
    read_register!(read_motion_mask, MotionMask);
    read_register!(read_interrupt_pulse_width, InterruptPulseWidth);
    read_register!(read_normal_scan_period, NormalScanPeriod);
    read_register!(read_motion_sliding_angle, MotionSlidingAngle);
    read_register!(read_low_power_scan_raw1, LowPowerScanRaw1);
    read_register!(read_low_power_scan_raw2, LowPowerScanRaw2);
    read_register!(read_low_power_auto_wake_time, LowPowerAutoWakeTime);
    read_register!(read_low_power_scan_threshold, LowPowerScanThreshold);
    read_register!(read_low_power_scan_window, LowPowerScanWindow);
    read_register!(read_low_power_scan_frequency, LowPowerScanFrequency);
    read_register!(read_low_power_scan_current, LowPowerScanCurrent);
    read_register!(read_auto_sleep_time, AutoSleepTime);
    read_register!(read_interrupt_control, InterruptControl);
    read_register!(read_auto_reset, AutoReset);
    read_register!(read_long_press_time, LongPressTime);
    read_register!(read_io_control, IoControl);
    read_register!(read_auto_sleep_disable, AutoSleepDisable);

    write_register!(write_bpc0, Bpc0);
    write_register!(write_bpc1, Bpc1);
    write_register!(write_motion_mask, MotionMask);
    write_register!(write_interrupt_pulse_width, InterruptPulseWidth);
    write_register!(write_normal_scan_period, NormalScanPeriod);
    write_register!(write_motion_sliding_angle, MotionSlidingAngle);
    write_register!(write_low_power_scan_raw1, LowPowerScanRaw1);
    write_register!(write_low_power_scan_raw2, LowPowerScanRaw2);
    write_register!(write_low_power_auto_wake_time, LowPowerAutoWakeTime);
    write_register!(write_low_power_scan_threshold, LowPowerScanThreshold);
    write_register!(write_low_power_scan_window, LowPowerScanWindow);
    write_register!(write_low_power_scan_frequency, LowPowerScanFrequency);
    write_register!(write_low_power_scan_current, LowPowerScanCurrent);
    write_register!(write_auto_sleep_time, AutoSleepTime);
    write_register!(write_interrupt_control, InterruptControl);
    write_register!(write_auto_reset, AutoReset);
    write_register!(write_long_press_time, LongPressTime);
    write_register!(write_io_control, IoControl);
    write_register!(write_auto_sleep_disable, AutoSleepDisable);
}
