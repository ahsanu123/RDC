use bitflags::bitflags;

bitflags! {
    // address = 10H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct ADCXRawValueRegister: u16{
                               //FEDC_BA98_7654_3210
        const X_RAW_VALUE    = 0b1111_1111_1111_1111;

    }
}
