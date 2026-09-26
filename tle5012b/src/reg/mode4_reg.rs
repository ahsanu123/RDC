use bitflags::bitflags;

bitflags! {
    // address = 0EH
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct Mode4Register: u16{
                                                                //FEDC_BA98_7654_3210
        const OFFSET_TEMPERATURE_COEFFICIENT_FOR_X_COMPONENT  = 0b1111_1110_0000_0000;
        const HALL_SWITCH_MODE                                = 0b0000_0001_1110_0000;
        const PULSE_WIDTH_MODULATION_MODE                     = 0b0000_0000_0001_1000;
        const INTERFACE_MODE_ON_IFA_IFB_IFC                   = 0b0000_0000_0000_0011;
    }
}
