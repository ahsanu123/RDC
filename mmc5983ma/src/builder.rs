use typed_builder::TypedBuilder;

use crate::{Mmc5983ma, comproc::RegisterInterface};

/// Protocol-typed builder. Construction performs no I/O; call initialize first.
#[derive(TypedBuilder)]
pub struct Builder<Interface> {
    interface: Interface,
    /// Number of status reads allowed per conversion, separated by 1 ms.
    #[builder(default = core::num::NonZeroU16::new(20).unwrap())]
    poll_limit: core::num::NonZeroU16,
}

impl<Interface: RegisterInterface> Builder<Interface> {
    pub fn build_device(self) -> Mmc5983ma<Interface> {
        Mmc5983ma::new(self.interface, self.poll_limit)
    }
}
