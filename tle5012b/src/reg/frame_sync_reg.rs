use bitflags::bitflags;

bitflags! {
    // address = 05H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct FrameSyncRegister: u16{
                                                          //FEDC_BA98_7654_3210
        const FRAME_SYNCHRONIZATION_COUNTER_VALUE_WU    = 0b1111_1110_0000_0000;
        const TEMPERATURE_VALUE_RU                      = 0b0000_0001_1111_1111;
    }
}
