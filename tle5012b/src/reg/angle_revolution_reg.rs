use bitflags::bitflags;

bitflags! {
    // address = 04H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct AngleRevolutionRegister: u16{
                                                  //FEDC_BA98_7654_3210
        const READ_STATUS_REVOLUTION            = 0b1000_0000_0000_0000;
        const FRAME_COUNTER_U6BIT_WU            = 0b0111_1110_0000_0000;
        const NUMBER_OF_REVOLUTION_SBIT_SRU     = 0b0000_0001_1111_1111;
    }
}
