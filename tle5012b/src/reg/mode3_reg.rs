use bitflags::bitflags;

bitflags! {
    // address = 09H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct Mode3Register: u16{
                                                        //FEDC_BA98_7654_3210
        const ANGLE_BASE_W                            = 0b1111_1111_1111_0000;
        const ANALOG_SPIKE_FILTER_OF_INPUT_PADS_W     = 0b0000_0000_0000_1000;
        const SSC_INTERFACE_DATA_PIN_OUTPUT_MODE_W    = 0b0000_0000_0000_0100;
        const CONFIGURATION_OF_PAD_DRIVER_W           = 0b0000_0000_0000_0011;
    }
}
