use bitflags::bitflags;

bitflags! {
    #[derive(PartialEq, Eq)]
    pub struct Numwires: u8 {
        const MAX31865_3_WIRE= 1;
        const MAX31865_2_OR_4_WIRE= 0;
    }
}
