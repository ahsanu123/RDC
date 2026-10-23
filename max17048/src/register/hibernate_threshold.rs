use bilge::prelude::*;

/// HIBRT, datasheet Rev. 7, figure 9.
#[bitsize(16)]
#[derive(Clone, Copy, DebugBits, FromBits, PartialEq, Eq)]
pub struct HibernateThreshold {
    pub activity_threshold: u8,
    pub hibernate_threshold: u8,
}
super::readable!(HibernateThreshold, 0x0a);
