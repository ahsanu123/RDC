use bitflags::bitflags;

bitflags! {
    // address = 30H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct Temperature25OffsetValueRegister: u16{
                                            //FEDC_BA98_7654_3210
        const TEMPERATURE_25_OFFSET_VALUE = 0b1111_1110_0000_0000;

    }
}
