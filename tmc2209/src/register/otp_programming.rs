use bilge::prelude::*;

/// OTP_PROG, irreversible bit-programming command. Rev. 1.09, section 5.1, page 24.
#[bitsize(32)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct OtpProgramming {
    pub bit_index: u3,
    pub unused_bit3: bool,
    pub byte_index: u2,
    pub unused_bits6_7: u2,
    pub programming_key: u8,
    pub unused_high: u16,
}

super::register!(OtpProgramming, 0x04, None);
