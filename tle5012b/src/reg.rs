// == Structure of the Command Word
//
// Name Bits Description
//
// RW [15] Read - Write
//     0: Write
//     1: Read
//
// Lock [14..11] 4-bit Lock Value
//     0000B: Default operating access for addresses 0x00:0x04, 0x14:0x15, 0x20, 0x30
//     1010B: Configuration access for addresses 0x05:0x11
//
// UPD [10] Update-Register Access
//     0: Access to current values
//     1: Access to values in update buffer
//
// ADDR [9..4] 6-bit Address
//
// ND [3..0] 4-bit Number of Data Words (if bits set to 0000B, no safety word is provided)
//
// ----------------------------------------------------------------------------------------
//
// data transfer (data-read example)
//      <COMMAND> [...delay...] <READ Data 1> <READ Data 2> <SAFETY-WORD>
//
// data transfer (data-write example)
//      <COMMAND> <WRITE Data 1> [...delay...] <SAFETY-WORD>
pub mod activation_status_reg;
pub mod adcx_raw_val;
pub mod adcy_raw_val;
pub mod angle_revolution_reg;
pub mod angle_speed_reg;
pub mod angle_value_reg;
pub mod angle_vector_magnitude;
pub mod frame_sync_reg;
pub mod ifab_reg;
pub mod iif_counter_val;
pub mod mode1_reg;
pub mod mode2_reg;
pub mod mode3_reg;
pub mod mode4_reg;
pub mod offset_x;
pub mod offset_y;
pub mod sil_reg;
pub mod status_reg;
pub mod synchronicity;
pub mod temperature25_offset_val;
pub mod temperature_coefficient_reg;
pub mod temperature_sensor_raw_val;

pub mod prelude {

    use crate::communication::possible_address::PossibleAddress;

    pub use super::activation_status_reg::*;
    pub use super::adcx_raw_val::*;
    pub use super::adcy_raw_val::*;
    pub use super::angle_revolution_reg::*;
    pub use super::angle_speed_reg::*;
    pub use super::angle_value_reg::*;
    pub use super::angle_vector_magnitude::*;
    pub use super::frame_sync_reg::*;
    pub use super::ifab_reg::*;
    pub use super::iif_counter_val::*;
    pub use super::mode1_reg::*;
    pub use super::mode2_reg::*;
    pub use super::mode3_reg::*;
    pub use super::mode4_reg::*;
    pub use super::offset_x::*;
    pub use super::offset_y::*;
    pub use super::sil_reg::*;
    pub use super::status_reg::*;
    pub use super::synchronicity::*;
    pub use super::temperature_coefficient_reg::*;
    pub use super::temperature_sensor_raw_val::*;
    pub use super::temperature25_offset_val::*;

    pub trait RegisterFromRaw: From<u16> {
        const ADDRESS: PossibleAddress;
        fn into_u16(self) -> u16;
    }

    pub enum PossibleRegister {
        StatusRegister(StatusRegisterStructure),
        ActivationStatusRegister(ActivationStatusRegisterStructure),
        AngleValueRegister(AngleValueRegisterStructure),
        AngleSpeedRegister(AngleSpeedRegisterStructure),
        AngleRevolutionRegister(AngleRevolutionRegisterStructure),
        FrameSynchronizationRegister(FrameSyncRegisterStructure),
        Mode1Register(Mode1RegisterStructure),
        SilRegister(SILRegisterStructure),
        Mode2Register(Mode2RegisterStructure),
        Mode3Register(Mode3RegisterStructure),
        OffsetX(OffsetXRegisterStructure),
        OffsetY(OffsetYRegisterStructure),
        Synchronicity(SynchronicityRegisterStructure),
        IfabRegister(IFABRegisterStructure),
        Mode4Register(Mode4RegisterStructure),
        TemperatureCoefficientReg(TemperatureCoefficientRegisterStructure),
        AdcXRawValue(ADCXRawValueRegisterStructure),
        AdcYRawValue(ADCYRawValueRegisterStructure),
        AngleVectorMagnitude(AngleVectorMagnitudeRegisterStructure),
        TemperatureSensorRawValue(TemperatureSensorRawValueRegisterStructure),
        IifCounterValue(IIFCounterValueRegisterStructure),
        Temperature25cOffsetValue(Temperature25OffsetValueRegisterStructure),
    }
}
