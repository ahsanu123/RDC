use bilge::prelude::*;

// STAT
//     Chip and Interface Status
//     [15] Indication of chip reset or watchdog overflow (resets after readout) via SSC
//         0: Reset occurred
//         1: No reset
//     [14] System error (e.g. overvoltage; undervoltage; VDD-, GND- off; ROM;...)
//         0: Error occurred (S_VR; S_DSPU; S_OV; S_XYOL: S_MAGOL; S_FUSE;  S_ROM; S_ADCT)
//         1: No error
//     [13] Interface access error (access to wrong address; wrong lock)
//         0: Error occurred
//         1: No error
//     [12] Invalid angle value (NO_GMR_A = 1; NO_GMR_XY = 1)
//         0: Angle value invalid
//         1: Angle value valid
// --------------------------------------------------------------------------------------
//     RESP [11..8] Sensor number response indicator
//         The sensor number bit is pulled low and the other bits are high (one-cold
//         encoding, e.g. for the sensor -or slave- number “00” the RESP bits are
//         “1110”. For the sensor -or slave- number “10” the RESP bits are “1011”).
//     CRC [7..0] Cyclic Redundancy Check (CRC), which includes the STAT and RESP bits

#[bitsize(16)]
#[derive(FromBits)]
// LSB first
pub struct SafetyWord {
    crc: u8,
    response: u4,
    chip_status: ChipStatus,
}

#[bitsize(4)]
#[derive(FromBits)]
// LSB first
pub struct ChipStatus {
    invalid_angle_value: InvalidAngleValue,
    interface_access_error: InterfaceAccessError,
    system_error: SystemError,
    chip_reset_or_watchdog_overflow: ChipResetOrWatchDogOverflow,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum ChipResetOrWatchDogOverflow {
    ResetOccurred = 0,
    NoReset,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum SystemError {
    ErrorOccurred = 0,
    NoError,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum InterfaceAccessError {
    ErrorOccurred = 0,
    NoError,
}

#[bitsize(1)]
#[derive(FromBits)]
pub enum InvalidAngleValue {
    AngleValueInvalid = 0,
    AngleValueValid,
}
