use bitflags::bitflags;

bitflags! {
    // address = 03H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct AngleValueRegister: u16{
                                                //FEDC_BA98_7654_3210
        const READ_STATUS_ANGLE_VALUE           = 0b1000_0000_0000_0000;
        const CALCULATED_ANGLE_VALUE_15BIT_RU   = 0b0111_1111_1111_1111;
    }
}
