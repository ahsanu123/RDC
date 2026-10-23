/// Device validation failures and the original SPI error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<InterfaceError> {
    Interface(InterfaceError),
    InvalidSpiFrequency {
        actual_hz: u32,
        minimum_hz: u32,
        maximum_hz: u32,
    },
    BufferTooSmall {
        required: usize,
        provided: usize,
    },
    FrameTooLarge,
}
