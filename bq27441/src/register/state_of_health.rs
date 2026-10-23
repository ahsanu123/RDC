use bilge::prelude::*;

/// StateOfHealth() high-byte status; SLUUAC9A section 4.15.
#[bitsize(8)]
#[derive(Clone, Copy, Debug, FromBits, PartialEq, Eq)]
pub enum HealthStatus {
    Invalid,
    Instant,
    Initial,
    Learned,
    #[fallback]
    Reserved(u8),
}

/// StateOfHealth() percentage and validity; SLUUAC9A section 4.15.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct StateOfHealth {
    pub percent: u8,
    pub status: HealthStatus,
}

super::register!(StateOfHealth, 0x20, None);
