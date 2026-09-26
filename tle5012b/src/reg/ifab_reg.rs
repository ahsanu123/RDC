use bitflags::bitflags;

bitflags! {
    // address = 0DH
    //
    // ru = read or upate
    // r = readonly
    // w = writeable
    // wu = writeable and update
    pub struct IFABRegister: u16{
                                                                 //FEDC_BA98_7654_3210
        const ORTHOGONALITY_CORRECTION_OF_X_AND_Y_COMPONENTS_W = 0b1111_1111_1111_0000;
        const FIR_UPDATE_RATE                                  = 0b0000_0000_0000_1000;
        const IFA_IFB_IFC_OUTPUT_MODE                          = 0b0000_0000_0000_0100;
        const HSM_AND_IIF_MODE                                 = 0b0000_0000_0000_0011;
    }
}
