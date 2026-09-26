use bitflags::bitflags;

bitflags! {
    // address = 08H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct Mode2Register: u16{
                                      //FEDC_BA98_7654_3210
        const ANGLE_RANGE_W           = 0b0111_1111_1111_0000;
        const ANGLE_DIRECTION_W       = 0b0000_0000_0000_1000;
        const PREDICTION_W            = 0b0000_0000_0000_0100;
        const AUTOCALIBRATION_MODE_W  = 0b0000_0000_0000_0011;
    }
}
