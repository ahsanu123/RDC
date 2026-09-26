use bitflags::bitflags;

bitflags! {
    // address = 01H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct ActivationStatusRegister: u16{
                                                                  //FEDC_BA98_7654_3210
        const ACTIVATION_OF_FIRMWARE_RESET_WU                   = 0b0000_0100_0000_0000;
        const EN_ADC_TEST_VECTOR_CHECK_WU                       = 0b0000_0010_0000_0000;
        const ACTIVATION_MAGNITUDE_CHECK_WU                     = 0b0000_0000_1000_0000;
        const ACTIVATION_XY_OUT_LIMIT_CHECK_WU                  = 0b0000_0000_0100_0000;
        const ENABLE_OF_DSPU_OVERFLOW_CHECK_WU                  = 0b0000_0000_0010_0000;

        const ACTIVATION_DSPU_BIST                              = 0b0000_0000_0001_0000;
        const ACTIVATION_FUSE_CRC                               = 0b0000_0000_0000_1000;
        const ENABLE_VOLTAGE_REGULATOR_CHECK                    = 0b0000_0000_0000_0100;
        const ENABLE_DSPU_WATCHDOG                              = 0b0000_0000_0000_0010;
        const ACTIVATION_OF_HARDWARE_RESET                      = 0b0000_0000_0000_0001;
    }
}
