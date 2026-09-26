use bitflags::bitflags;

bitflags! {
    // address = 20H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct TemperatureSensorRawValueRegister: u16{
                                           //FEDC_BA98_7654_3210
        const COUNTER_VALUE_OF_INCREMENT = 0b0011_1111_1111_1111;

    }
}
