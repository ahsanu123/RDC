use super::*;
use crate::{comproc, error::Cst816sError};
use embedded_hal::i2c::I2c;

/// Stateless register access; the caller owns and lends the bus for each method.
pub(crate) struct Register;

macro_rules! read_byte {
    ($method:ident, $register:ident) => {
        pub(crate) fn $method<I: I2c>(
            &self,
            i2c: &mut I,
            device_address: u8,
        ) -> Result<$register, Cst816sError<I::Error>> {
            comproc::read(i2c, device_address, $register::ADDRESS).map(Into::into)
        }
    };
}

macro_rules! read_checked_byte {
    ($method:ident, $register:ident) => {
        pub(crate) fn $method<I: I2c>(
            &self,
            i2c: &mut I,
            device_address: u8,
        ) -> Result<$register, Cst816sError<I::Error>> {
            let value = comproc::read(i2c, device_address, $register::ADDRESS)?;
            $register::try_from(value).map_err(|_| Cst816sError::InvalidRegisterValue {
                address: $register::ADDRESS,
                value,
            })
        }
    };
}

macro_rules! write_byte {
    ($method:ident, $register:ident) => {
        pub(crate) fn $method<I: I2c>(
            &self,
            i2c: &mut I,
            device_address: u8,
            value: $register,
        ) -> Result<(), Cst816sError<I::Error>> {
            comproc::write(i2c, device_address, $register::ADDRESS, value.into())
        }
    };
}

macro_rules! write_masked_byte {
    ($method:ident, $register:ident, $mask:expr) => {
        pub(crate) fn $method<I: I2c>(
            &self,
            i2c: &mut I,
            device_address: u8,
            value: $register,
        ) -> Result<(), Cst816sError<I::Error>> {
            let current = comproc::read(i2c, device_address, $register::ADDRESS)?;
            let updated = (current & !$mask) | (u8::from(value) & $mask);
            comproc::write(i2c, device_address, $register::ADDRESS, updated)
        }
    };
}

macro_rules! read_pair {
    ($method:ident, $register:ident, $high:ident, $low:ident) => {
        pub(crate) fn $method<I: I2c>(
            &self,
            i2c: &mut I,
            device_address: u8,
        ) -> Result<$register, Cst816sError<I::Error>> {
            let high = comproc::read(i2c, device_address, Address::$high)?;
            let low = comproc::read(i2c, device_address, Address::$low)?;
            Ok(u16::from_be_bytes([high, low]).into())
        }
    };
}

macro_rules! write_pair {
    ($method:ident, $register:ident, $high:ident, $low:ident) => {
        pub(crate) fn $method<I: I2c>(
            &self,
            i2c: &mut I,
            device_address: u8,
            value: $register,
        ) -> Result<(), Cst816sError<I::Error>> {
            let [high, low] = u16::from(value).to_be_bytes();
            comproc::write(i2c, device_address, Address::$high, high)?;
            comproc::write(i2c, device_address, Address::$low, low)
        }
    };
}

impl Register {
    read_checked_byte!(read_gesture_identifier, GestureIdentifier);
    read_checked_byte!(read_finger_number, FingerNumber);
    read_pair!(read_x_position, XPosition, XPositionHigh, XPositionLow);
    read_pair!(read_y_position, YPosition, YPositionHigh, YPositionLow);
    read_byte!(read_chip_identifier, ChipIdentifier);
    read_byte!(read_project_identifier, ProjectIdentifier);
    read_byte!(read_firmware_version, FirmwareVersion);
    read_pair!(read_bpc0, Bpc0, Bpc0High, Bpc0Low);
    read_pair!(read_bpc1, Bpc1, Bpc1High, Bpc1Low);
    read_byte!(read_motion_mask, MotionMask);
    read_byte!(read_interrupt_pulse_width, InterruptPulseWidth);
    read_byte!(read_normal_scan_period, NormalScanPeriod);
    read_byte!(read_motion_sliding_angle, MotionSlidingAngle);
    read_pair!(
        read_low_power_scan_raw1,
        LowPowerScanRaw1,
        LowPowerScanRaw1High,
        LowPowerScanRaw1Low
    );
    read_pair!(
        read_low_power_scan_raw2,
        LowPowerScanRaw2,
        LowPowerScanRaw2High,
        LowPowerScanRaw2Low
    );
    read_byte!(read_low_power_auto_wake_time, LowPowerAutoWakeTime);
    read_byte!(read_low_power_scan_threshold, LowPowerScanThreshold);
    read_byte!(read_low_power_scan_window, LowPowerScanWindow);
    read_byte!(read_low_power_scan_frequency, LowPowerScanFrequency);
    read_byte!(read_low_power_scan_current, LowPowerScanCurrent);
    read_byte!(read_auto_sleep_time, AutoSleepTime);
    read_byte!(read_interrupt_control, InterruptControl);
    read_byte!(read_auto_reset, AutoReset);
    read_byte!(read_long_press_time, LongPressTime);
    read_byte!(read_io_control, IoControl);
    read_byte!(read_auto_sleep_disable, AutoSleepDisable);

    write_pair!(write_bpc0, Bpc0, Bpc0High, Bpc0Low);
    write_pair!(write_bpc1, Bpc1, Bpc1High, Bpc1Low);
    write_masked_byte!(write_motion_mask, MotionMask, 0x07);
    write_byte!(write_interrupt_pulse_width, InterruptPulseWidth);
    write_byte!(write_normal_scan_period, NormalScanPeriod);
    write_byte!(write_motion_sliding_angle, MotionSlidingAngle);
    write_pair!(
        write_low_power_scan_raw1,
        LowPowerScanRaw1,
        LowPowerScanRaw1High,
        LowPowerScanRaw1Low
    );
    write_pair!(
        write_low_power_scan_raw2,
        LowPowerScanRaw2,
        LowPowerScanRaw2High,
        LowPowerScanRaw2Low
    );
    write_byte!(write_low_power_auto_wake_time, LowPowerAutoWakeTime);
    write_byte!(write_low_power_scan_threshold, LowPowerScanThreshold);
    write_byte!(write_low_power_scan_window, LowPowerScanWindow);
    write_byte!(write_low_power_scan_frequency, LowPowerScanFrequency);
    write_byte!(write_low_power_scan_current, LowPowerScanCurrent);
    write_byte!(write_auto_sleep_time, AutoSleepTime);
    write_masked_byte!(write_interrupt_control, InterruptControl, 0xF1);
    write_byte!(write_auto_reset, AutoReset);
    write_byte!(write_long_press_time, LongPressTime);
    write_masked_byte!(write_io_control, IoControl, 0x07);
    write_byte!(write_auto_sleep_disable, AutoSleepDisable);
}
