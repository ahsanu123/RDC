use bitflags::bitflags;

bitflags! {
    // address = 0FH
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct TemperatureCoefficientRegister: u16{
                                                                  //FEDC_BA98_7654_3210
        const OFFSET_TEMPERATURE_COEFFICIENT_FOR_Y_COMPONENT    = 0b1111_1100_0000_0000;
        const STARTUP_BIST                                      = 0b0000_0001_0000_0000;
        const CRC_OF_PARAMETERS                                 = 0b0000_0000_1111_1111;

    }
}
