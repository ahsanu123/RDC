use bitflags::bitflags;

bitflags! {
    // address = 0BH
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct OffsetYRegister: u16{
                                                         //FEDC_BA98_7654_3210
        const OFFSET_CORRECTION_OF_Y_VALUE_IN_DIGITS_W = 0b1111_1111_1111_0000;
    }
}
