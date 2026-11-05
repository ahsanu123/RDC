//! Typed byte registers from CST816S_register_declaration.pdf, pages 1-5.
//!
//! Fields follow bilge's least-significant-first order. Unspecified bits are
//! retained when decoding and modifying a register. Defaults are implemented
//! only where the PDF specifies the complete byte's default.
//!
//! These types do not establish access permissions. The private Register helper
//! performs transfers using an I2C reference supplied by the owning device.
//! Gesture and finger-count decoding is fallible for undocumented codes.

mod access;
mod auto_reset;
mod auto_sleep_disable;
mod auto_sleep_time;
mod bpc0;
mod bpc0_high;
mod bpc0_low;
mod bpc1;
mod bpc1_high;
mod bpc1_low;
mod chip_identifier;
mod finger_number;
mod firmware_version;
mod gesture_identifier;
mod interrupt_control;
mod interrupt_pulse_width;
mod io_control;
mod long_press_time;
mod low_power_auto_wake_time;
mod low_power_scan_current;
mod low_power_scan_frequency;
mod low_power_scan_raw1;
mod low_power_scan_raw1_high;
mod low_power_scan_raw1_low;
mod low_power_scan_raw2;
mod low_power_scan_raw2_high;
mod low_power_scan_raw2_low;
mod low_power_scan_threshold;
mod low_power_scan_window;
mod motion_mask;
mod motion_sliding_angle;
mod normal_scan_period;
mod project_identifier;
mod x_position;
mod x_position_high;
mod x_position_low;
mod y_position;
mod y_position_high;
mod y_position_low;

pub(crate) use access::Register;
pub use auto_reset::AutoReset;
pub use auto_sleep_disable::AutoSleepDisable;
pub use auto_sleep_time::AutoSleepTime;
pub use bpc0::Bpc0;
pub use bpc0_high::Bpc0High;
pub use bpc0_low::Bpc0Low;
pub use bpc1::Bpc1;
pub use bpc1_high::Bpc1High;
pub use bpc1_low::Bpc1Low;
pub use chip_identifier::ChipIdentifier;
pub use finger_number::{FingerCount, FingerNumber};
pub use firmware_version::FirmwareVersion;
pub use gesture_identifier::{Gesture, GestureIdentifier};
pub use interrupt_control::InterruptControl;
pub use interrupt_pulse_width::InterruptPulseWidth;
pub use io_control::{IoControl, IoVoltage, TwoWireDriverMode};
pub use long_press_time::LongPressTime;
pub use low_power_auto_wake_time::LowPowerAutoWakeTime;
pub use low_power_scan_current::LowPowerScanCurrent;
pub use low_power_scan_frequency::LowPowerScanFrequency;
pub use low_power_scan_raw1::LowPowerScanRaw1;
pub use low_power_scan_raw1_high::LowPowerScanRaw1High;
pub use low_power_scan_raw1_low::LowPowerScanRaw1Low;
pub use low_power_scan_raw2::LowPowerScanRaw2;
pub use low_power_scan_raw2_high::LowPowerScanRaw2High;
pub use low_power_scan_raw2_low::LowPowerScanRaw2Low;
pub use low_power_scan_threshold::LowPowerScanThreshold;
pub use low_power_scan_window::LowPowerScanWindow;
pub use motion_mask::MotionMask;
pub use motion_sliding_angle::MotionSlidingAngle;
pub use normal_scan_period::NormalScanPeriod;
pub use project_identifier::ProjectIdentifier;
pub use x_position::XPosition;
pub use x_position_high::XPositionHigh;
pub use x_position_low::XPositionLow;
pub use y_position::YPosition;
pub use y_position_high::YPositionHigh;
pub use y_position_low::YPositionLow;

/// Register addresses declared in CST816S_register_declaration.pdf, pages 1-5.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Address {
    GestureIdentifier = 0x01,
    FingerNumber = 0x02,
    XPositionHigh = 0x03,
    XPositionLow = 0x04,
    YPositionHigh = 0x05,
    YPositionLow = 0x06,
    ChipIdentifier = 0xA7,
    ProjectIdentifier = 0xA8,
    FirmwareVersion = 0xA9,
    Bpc0High = 0xB0,
    Bpc0Low = 0xB1,
    Bpc1High = 0xB2,
    Bpc1Low = 0xB3,
    MotionMask = 0xEC,
    InterruptPulseWidth = 0xED,
    NormalScanPeriod = 0xEE,
    MotionSlidingAngle = 0xEF,
    LowPowerScanRaw1High = 0xF0,
    LowPowerScanRaw1Low = 0xF1,
    LowPowerScanRaw2High = 0xF2,
    LowPowerScanRaw2Low = 0xF3,
    LowPowerAutoWakeTime = 0xF4,
    LowPowerScanThreshold = 0xF5,
    LowPowerScanWindow = 0xF6,
    LowPowerScanFrequency = 0xF7,
    LowPowerScanCurrent = 0xF8,
    AutoSleepTime = 0xF9,
    InterruptControl = 0xFA,
    AutoReset = 0xFB,
    LongPressTime = 0xFC,
    IoControl = 0xFD,
    AutoSleepDisable = 0xFE,
}

macro_rules! register_metadata {
    ($($register:ident),+ $(,)?) => {
        $(
            impl $register {
                /// Address of this byte register.
                pub const ADDRESS: Address = Address::$register;
            }

            #[cfg(feature = "defmt")]
            impl defmt::Format for $register {
                fn format(&self, formatter: defmt::Formatter) {
                    defmt::write!(
                        formatter,
                        "{}({=u8:#x})",
                        stringify!($register),
                        u8::from(*self)
                    );
                }
            }
        )+
    };
}

register_metadata!(
    GestureIdentifier,
    FingerNumber,
    XPositionHigh,
    XPositionLow,
    YPositionHigh,
    YPositionLow,
    ChipIdentifier,
    ProjectIdentifier,
    FirmwareVersion,
    Bpc0High,
    Bpc0Low,
    Bpc1High,
    Bpc1Low,
    MotionMask,
    InterruptPulseWidth,
    NormalScanPeriod,
    MotionSlidingAngle,
    LowPowerScanRaw1High,
    LowPowerScanRaw1Low,
    LowPowerScanRaw2High,
    LowPowerScanRaw2Low,
    LowPowerAutoWakeTime,
    LowPowerScanThreshold,
    LowPowerScanWindow,
    LowPowerScanFrequency,
    LowPowerScanCurrent,
    AutoSleepTime,
    InterruptControl,
    AutoReset,
    LongPressTime,
    IoControl,
    AutoSleepDisable,
);

macro_rules! combined_register_metadata {
    ($($register:ident => $address:ident),+ $(,)?) => {
        $(
            impl $register {
                /// Address of the high byte of this combined register.
                pub const ADDRESS: Address = Address::$address;
            }

            #[cfg(feature = "defmt")]
            impl defmt::Format for $register {
                fn format(&self, formatter: defmt::Formatter) {
                    defmt::write!(
                        formatter,
                        "{}({=u16:#x})",
                        stringify!($register),
                        u16::from(*self)
                    );
                }
            }
        )+
    };
}

combined_register_metadata!(
    XPosition => XPositionHigh,
    YPosition => YPositionHigh,
    Bpc0 => Bpc0High,
    Bpc1 => Bpc1High,
    LowPowerScanRaw1 => LowPowerScanRaw1High,
    LowPowerScanRaw2 => LowPowerScanRaw2High,
);
