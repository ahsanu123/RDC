use bitflags::bitflags;

bitflags! {
    // address = 07H
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct SILRegister: u16{
                                   //FEDC_BA98_7654_3210
        const FILTER_PARALLEL_W  = 0b1000_0000_0000_0000;
        const FILTER_INVERTED_W  = 0b0100_0000_0000_0000;
        const FUSE_RELOAD_W      = 0b0000_0100_0000_0000;
        const ADC_TEST_VECTORS_W = 0b0000_0000_0100_0000;
        const TEST_VECTOR_Y_W    = 0b0000_0000_0011_1000;
        const TEST_VECTOR_X_W    = 0b0000_0000_0000_0111;
    }
}
