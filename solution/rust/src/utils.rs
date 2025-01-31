use crate::{BitcoinError, Result};

pub fn encode_varint(value: u64) -> Vec<u8> {
    let mut result = Vec::new();
    if value < 0xFD {
        result.push(value as u8);
    } else if value <= 0xFFFF {
        result.push(0xFD);
        result.extend_from_slice(&(value as u16).to_le_bytes());
    } else if value <= 0xFFFFFFFF {
        result.push(0xFE);
        result.extend_from_slice(&(value as u32).to_le_bytes());
    } else {
        result.push(0xFF);
        result.extend_from_slice(&(value as u64).to_le_bytes());
    }
    result
}

pub fn decode_varint(payload: &[u8]) -> Result<(u64, usize)> {
    if payload.is_empty() {
        return Err(BitcoinError::InvalidPayload("Payload is empty".to_string()));
    }

    let first_byte = payload[0];
    match first_byte {
        0xFF => {
            if payload.len() < 9 {
                return Err(BitcoinError::InvalidPayload(
                    "Insufficient bytes for uint64".to_string(),
                ));
            }
            let value = u64::from_le_bytes(payload[1..9].try_into().unwrap());
            Ok((value, 9))
        }
        0xFE => {
            if payload.len() < 5 {
                return Err(BitcoinError::InvalidPayload(
                    "Insufficient bytes for uint32".to_string(),
                ));
            }
            let value = u32::from_le_bytes(payload[1..5].try_into().unwrap()) as u64;
            Ok((value, 5))
        }
        0xFD => {
            if payload.len() < 3 {
                return Err(BitcoinError::InvalidPayload(
                    "Insufficient bytes for uint16".to_string(),
                ));
            }
            let value = u16::from_le_bytes(payload[1..3].try_into().unwrap()) as u64;
            Ok((value, 3))
        }
        _ => {
            // Value is stored directly in the first byte.
            Ok((first_byte as u64, 1))
        }
    }
}

pub fn bits_to_target(bits: u32) -> [u8; 32] {
    let mut target = [0u8; 32];

    let size = (bits >> 24) as usize;
    let mut mantissa = bits & 0x007fffff;

    if mantissa > 0x7fffff {
        mantissa = 0x7fffff;
    }

    let start_pos = if size <= 32 { 32 - size } else { 0 };

    if start_pos + 2 < 32 {
        target[start_pos] = ((mantissa >> 16) & 0xff) as u8;
        target[start_pos + 1] = ((mantissa >> 8) & 0xff) as u8;
        target[start_pos + 2] = (mantissa & 0xff) as u8;
    }

    target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits_to_target() {
        // Test case 1: Normal case
        let bits = 0x1b0404cb;
        let expected = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x04, 0xcb, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];
        assert_eq!(bits_to_target(bits), expected);
    }
}
