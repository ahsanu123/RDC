use bitflags::bitflags;

bitflags! {
    // address = 0CH
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct Synchronicity: u16{
                                          //FEDC_BA98_7654_3210
        const AMPLITUDE_SYNCHRONICITY_W = 0b1111_1111_1111_0000;
    }
}
