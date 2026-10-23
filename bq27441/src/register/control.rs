use bilge::prelude::*;

/// Control() payload or selected response; SLUUAC9A section 4.1.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct Control {
    pub value: u16,
}

/// CONTROL_STATUS response; SLUUAC9A table 4-3, low bit first.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct ControlStatus {
    pub reserved0: bool,
    pub voltage_ok: bool,
    pub resistance_updates_disabled: bool,
    pub constant_power_model: bool,
    pub sleeping: bool,
    pub reserved5: bool,
    pub hibernate_requested: bool,
    pub initialization_complete: bool,
    pub resistance_updated: bool,
    pub maximum_capacity_updated: bool,
    pub board_calibration_active: bool,
    pub coulomb_counter_calibration_active: bool,
    pub calibration_mode: bool,
    pub sealed: bool,
    pub watchdog_reset: bool,
    pub shutdown_enabled: bool,
}

/// Supported Control() subcommands; SLUUAC9A table 4-2.
#[bitsize(16)]
#[derive(Clone, Copy, Debug, TryFromBits, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ControlCommand {
    Status = 0x0000,
    DeviceType = 0x0001,
    FirmwareVersion = 0x0002,
    DataMemoryCode = 0x0004,
    PreviousCommand = 0x0007,
    ChemistryIdentifier = 0x0008,
    BatteryInsert = 0x000c,
    BatteryRemove = 0x000d,
    SetHibernate = 0x0011,
    ClearHibernate = 0x0012,
    SetConfigurationUpdate = 0x0013,
    EnableShutdown = 0x001b,
    Shutdown = 0x001c,
    Seal = 0x0020,
    PulseStateOfChargeInterrupt = 0x0023,
    Reset = 0x0041,
    SoftReset = 0x0042,
    ExitConfigurationUpdate = 0x0043,
    ExitConfigurationUpdateWithResimulation = 0x0044,
}

impl ControlCommand {
    pub(crate) fn has_response(self) -> bool {
        matches!(
            self,
            Self::Status
                | Self::DeviceType
                | Self::FirmwareVersion
                | Self::DataMemoryCode
                | Self::PreviousCommand
                | Self::ChemistryIdentifier
        )
    }

    pub(crate) fn requires_unsealed(self) -> bool {
        matches!(
            self,
            Self::SetConfigurationUpdate
                | Self::EnableShutdown
                | Self::Shutdown
                | Self::Reset
                | Self::SoftReset
                | Self::ExitConfigurationUpdate
                | Self::ExitConfigurationUpdateWithResimulation
        )
    }
}
