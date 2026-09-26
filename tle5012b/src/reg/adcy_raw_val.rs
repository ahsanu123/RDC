use bitflags::bitflags;

bitflags! {
    // address = 11H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct ADCYRawValueRegister: u16{
                               //FEDC_BA98_7654_3210
        const Y_RAW_VALUE    = 0b1111_1111_1111_1111;

    }
}
