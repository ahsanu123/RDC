use bitflags::bitflags;

bitflags! {
    // address = 15H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct TemperatureSensorRawValue: u16{
                                                          //FEDC_BA98_7654_3210
        const TEMPERATURE_SENSOR_RAW_VALUE_TOGGLE_RU    = 0b1000_0000_0000_0000;
        const TEMPERATURE_SENSOR_RAW_VALUE_RU           = 0b0000_0011_1111_1111;

    }
}
