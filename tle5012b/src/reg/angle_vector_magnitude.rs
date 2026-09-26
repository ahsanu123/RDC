use bitflags::bitflags;

bitflags! {
    // address = 11H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct AngleVectorMagnitued: u16{
                                           //FEDC_BA98_7654_3210
        const ANGLE_VECTOR_MAGNITUDE_RU  = 0b0000_0011_1111_1111;

    }
}
