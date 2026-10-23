use crate::register::{BlockData, BlockDataChecksum};

/// SLUUAC9A section 5.6: complemented eight-bit sum over the whole page.
pub(crate) fn checksum(block: BlockData) -> BlockDataChecksum {
    let sum = block
        .bytes
        .iter()
        .fold(0u8, |sum, byte| sum.wrapping_add(byte.value()));
    BlockDataChecksum::from(255 - sum)
}

pub(crate) fn validate_checksum(
    block: BlockData,
    received: BlockDataChecksum,
) -> Result<(), (u8, u8)> {
    let expected = checksum(block).value();
    if expected == received.value() {
        Ok(())
    } else {
        Err((expected, received.value()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum_full_page_wrapping_and_validation() {
        for (bytes, expected) in [([0; 32], 255), ([255; 32], 31), ([0x55; 32], 95)] {
            let block = BlockData::from(bytes);
            assert_eq!(checksum(block).value(), expected);
            assert_eq!(validate_checksum(block, expected.into()), Ok(()));
            assert_eq!(
                validate_checksum(block, expected.wrapping_add(1).into()),
                Err((expected, expected.wrapping_add(1)))
            );
        }
    }
}
