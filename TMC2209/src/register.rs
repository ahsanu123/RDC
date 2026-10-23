//! Typed 32-bit register words; UART serializes these words most-significant byte first.

pub mod automatic_pwm_values;
pub mod chopper_configuration;
pub mod current_control;
pub mod driver_status;
pub mod factory_configuration;
pub mod general_configuration;
pub mod global_status;
pub mod input_status;
pub mod interface_counter;
pub mod microstep_counter;
pub mod microstep_current;
pub mod node_configuration;
pub mod otp_configuration;
pub mod otp_programming;
pub mod power_down_delay;
pub mod pwm_configuration;
pub mod pwm_scaling;
pub mod pwm_velocity_threshold;
pub mod smart_current_configuration;
pub mod smart_current_velocity_threshold;
pub mod stall_detection_result;
pub mod stall_detection_threshold;
pub mod step_interval;
pub mod target_velocity;

pub use automatic_pwm_values::AutomaticPwmValues;
pub use chopper_configuration::{ChopperConfiguration, MicrostepResolution};
pub use current_control::CurrentControl;
pub use driver_status::DriverStatus;
pub use factory_configuration::FactoryConfiguration;
pub use general_configuration::GeneralConfiguration;
pub use global_status::GlobalStatus;
pub use input_status::InputStatus;
pub use interface_counter::InterfaceCounter;
pub use microstep_counter::MicrostepCounter;
pub use microstep_current::MicrostepCurrent;
pub use node_configuration::NodeConfiguration;
pub use otp_configuration::OtpConfiguration;
pub use otp_programming::OtpProgramming;
pub use power_down_delay::PowerDownDelay;
pub use pwm_configuration::{PwmConfiguration, StandstillMode};
pub use pwm_scaling::PwmScaling;
pub use pwm_velocity_threshold::PwmVelocityThreshold;
pub use smart_current_configuration::SmartCurrentConfiguration;
pub use smart_current_velocity_threshold::SmartCurrentVelocityThreshold;
pub use stall_detection_result::StallDetectionResult;
pub use stall_detection_threshold::StallDetectionThreshold;
pub use step_interval::StepInterval;
pub use target_velocity::TargetVelocity;

mod sealed {
    pub trait Sealed {}
}

/// A register word; `None` means reset contents are live, OTP-dependent or unspecified.
pub trait Register: sealed::Sealed + From<u32> + Into<u32> + Copy {
    const ADDRESS: u8;
    const RESET: Option<u32>;
}

/// Registers supporting ordinary UART reads.
///
/// ```compile_fail
/// use tmc2209::register::{CurrentControl, ReadableRegister};
/// fn readable<R: ReadableRegister>() {}
/// readable::<CurrentControl>();
/// ```
pub trait ReadableRegister: Register {}

/// Registers writable through the checked device API, excluding irreversible OTP commands.
///
/// ```compile_fail
/// use tmc2209::register::{DriverStatus, WritableRegister};
/// fn writable<R: WritableRegister>() {}
/// writable::<DriverStatus>();
/// ```
pub trait WritableRegister: Register {
    const WRITE_MASK: u32;
}

macro_rules! register {
    ($name:ty, $address:expr, $reset:expr) => {
        impl super::sealed::Sealed for $name {}
        impl super::Register for $name {
            const ADDRESS: u8 = $address;
            const RESET: Option<u32> = $reset;
        }
    };
}

pub(crate) use register;
