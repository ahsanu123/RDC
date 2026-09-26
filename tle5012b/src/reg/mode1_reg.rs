use bitflags::bitflags;

bitflags! {
    // address = 06H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct Mode1Register: u16{
                                              //FEDC_BA98_7654_3210
        const UPDATE_RATE_SETTING_W         = 0b1100_0000_0000_0000;
        const CLOCK_SOURCE_SELECT_W         = 0b0000_0000_0001_0000;
        const HOLD_DSPU_OPERATION_W         = 0b0000_0000_0000_0100;
        const INCREMENTAL_INTERFACE_MODE_W  = 0b0000_0000_0000_0011;

    }
}
