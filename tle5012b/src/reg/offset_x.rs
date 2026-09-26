use bitflags::bitflags;

bitflags! {
    // address = 0AH
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct OffsetXRegister: u16{
                                                         //FEDC_BA98_7654_3210
        const OFFSET_CORRECTION_OF_X_VALUE_IN_DIGITS_W = 0b1111_1111_1111_0000;
    }
}
