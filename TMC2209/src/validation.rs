//! CRC8-ATM and UART reply checks, datasheet rev. 1.09 sections 4.1-4.2.

use crate::DeviceError;

const CRC_POLYNOMIAL: u8 = 0x07;
const SYNC_BYTE: u8 = 0x05;
const MASTER_ADDRESS: u8 = 0xff;

/// Compute the TMC UART CRC: zero seed, data bits LSB-first, CRC register shifted left.
pub fn crc8(bytes: &[u8]) -> u8 {
    let mut crc = 0_u8;
    for &byte in bytes {
        let mut current = byte;
        for _ in 0..8 {
            let feedback = (crc >> 7) ^ (current & 1);
            crc <<= 1;
            if feedback != 0 {
                crc ^= CRC_POLYNOMIAL;
            }
            current >>= 1;
        }
    }
    crc
}

pub(crate) fn validate_reply<E>(reply: &[u8; 8], register: u8) -> Result<u32, DeviceError<E>> {
    if reply[0] != SYNC_BYTE {
        return Err(DeviceError::InvalidReplySync { actual: reply[0] }.reported());
    }
    if reply[1] != MASTER_ADDRESS {
        return Err(DeviceError::InvalidReplyAddress { actual: reply[1] }.reported());
    }
    if reply[2] != register {
        return Err(DeviceError::UnexpectedRegister {
            expected: register,
            actual: reply[2],
        }
        .reported());
    }
    let expected = crc8(&reply[..7]);
    if reply[7] != expected {
        return Err(DeviceError::CrcMismatch {
            expected,
            actual: reply[7],
        }
        .reported());
    }
    Ok(u32::from_be_bytes([reply[3], reply[4], reply[5], reply[6]]))
}

#[cfg(test)]
mod tests {
    use core::convert::Infallible;

    use super::*;

    fn reply() -> [u8; 8] {
        let mut reply = [0x05, 0xff, 0x06, 0x21, 0x00, 0x00, 0x40, 0];
        reply[7] = crc8(&reply[..7]);
        reply
    }

    #[test]
    fn test_crc8_known_read_and_write_vectors() {
        assert_eq!(crc8(&[0x05, 0x00, 0x00]), 0x48);
        assert_eq!(crc8(&[0x05, 0x00, 0x06]), 0x6f);
        assert_eq!(crc8(&[0x05, 0x00, 0xec, 0x10, 0x00, 0x00, 0x53]), 0x9c);
        assert_eq!(crc8(&[0x05, 0x00, 0x90, 0x00, 0x0a, 0x1f, 0x03]), 0x86);
        assert_eq!(crc8(&[]), 0);
    }

    #[test]
    fn test_validate_reply_checks_identity_register_and_byte_order() {
        assert_eq!(
            validate_reply::<Infallible>(&reply(), 0x06),
            Ok(0x2100_0040)
        );
        let mut invalid = reply();
        invalid[0] = 0x04;
        assert_eq!(
            validate_reply::<Infallible>(&invalid, 0x06),
            Err(DeviceError::InvalidReplySync { actual: 0x04 })
        );
        invalid = reply();
        invalid[1] = 0;
        assert_eq!(
            validate_reply::<Infallible>(&invalid, 0x06),
            Err(DeviceError::InvalidReplyAddress { actual: 0 })
        );
        assert_eq!(
            validate_reply::<Infallible>(&reply(), 0x07),
            Err(DeviceError::UnexpectedRegister {
                expected: 7,
                actual: 6
            })
        );
    }

    #[test]
    fn test_validate_reply_detects_every_single_bit_data_corruption() {
        let valid = reply();
        for bit in 24..64 {
            let mut invalid = valid;
            invalid[bit / 8] ^= 1 << (bit % 8);
            assert!(matches!(
                validate_reply::<Infallible>(&invalid, 0x06),
                Err(DeviceError::CrcMismatch { .. })
            ));
        }
    }
}
