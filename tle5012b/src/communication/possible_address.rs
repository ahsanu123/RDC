use bilge::prelude::*;
// STAT        STATus register                 00H
// ACSTAT      ACtivation STATus register      01H
// AVAL        Angle VALue register            02H
// ASPD        Angle SPeeD register            03H
// AREV        Angle REVolution register       04H
// FSYNC       Frame SYNChronization register  05H
// MOD_1       Interface MODe1 register        06H
// SIL SIL     register                        07H
// MOD_2       Interface MODe2 register        08H
// MOD_3       Interface MODe3 register        09H
// OFFX        OFFset X                        0AH
// OFFY        OFFset Y                        0BH
// SYNCH       SYNCHronicity                   0CH
// IFAB IFAB   register                        0DH
// MOD_4       Interface MODe4 register        0EH
// TCO_Y       Temperature COefficient reg     0FH
// ADC_X       ADC X-raw value                 10H
// ADC_Y       ADC Y-raw value                 11H
// D_MAG       Angle vector MAGnitude          14H
// T_RAW       Temperature sensor RAW-value    15H
// IIF_CNT     IIF CouNTer value               20H
// T25O        Temperature 25°C Offset value   30H

#[bitsize(6)]
#[derive(PartialEq, Clone, TryFromBits)]
pub enum PossibleAddress {
    StatusRegister = 0x00,               // 00H
    ActivationStatusRegister = 0x01,     // 01H
    AngleValueRegister = 0x02,           // 02H
    AngleSpeedRegister = 0x03,           // 03H
    AngleRevolutionRegister = 0x04,      // 04H
    FrameSynchronizationRegister = 0x05, // 05H
    InterfaceMode1Register = 0x06,       // 06H
    SilRegister = 0x07,                  // 07H
    InterfaceMode2Register = 0x08,       // 08H
    InterfaceMode3Register = 0x09,       // 09H
    OffsetX = 0x0A,                      // 0AH
    OffsetY = 0x0B,                      // 0BH
    Synchronicity = 0x0C,                // 0CH
    IfabRegister = 0x0D,                 // 0DH
    InterfaceMode4Register = 0x0E,       // 0EH
    TemperatureCoefficientReg = 0x0F,    // 0FH
    AdcXRawValue = 0x10,                 // 10H
    AdcYRawValue = 0x11,                 // 11H
    AngleVectorMagnitude = 0x14,         // 14H
    TemperatureSensorRawValue = 0x15,    // 15H
    IifCounterValue = 0x20,              // 20H
    Temperature25cOffsetValue = 0x30,    // 30H
}
