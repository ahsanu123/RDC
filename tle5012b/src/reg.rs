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
pub(crate) mod activation_status_reg;
pub(crate) mod adcx_raw_val;
pub(crate) mod adcy_raw_val;
pub(crate) mod angle_revolution_reg;
pub(crate) mod angle_speed_reg;
pub(crate) mod angle_value_reg;
pub(crate) mod angle_vector_magnitude;
pub(crate) mod frame_sync_reg;
pub(crate) mod ifab_reg;
pub(crate) mod iif_counter_val;
pub(crate) mod mode1_reg;
pub(crate) mod mode2_reg;
pub(crate) mod mode3_reg;
pub(crate) mod mode4_reg;
pub(crate) mod offset_x;
pub(crate) mod offset_y;
pub(crate) mod sil_reg;
pub(crate) mod status_reg;
pub(crate) mod synchronicity;
pub(crate) mod temperature25_offset_val;
pub(crate) mod temperature_coefficient_reg;
pub(crate) mod temperature_sensor_raw_val;
