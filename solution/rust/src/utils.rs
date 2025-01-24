use sha2::{Digest, Sha256};

use crate::{BitcoinError, Result};

pub fn calculate_checksum(payload: &[u8]) -> [u8; 4] {
    let hash1 = Sha256::digest(payload);
    let hash2 = Sha256::digest(hash1);
    let mut checksum = [0u8; 4];
    checksum.copy_from_slice(&hash2[..4]);
    checksum
}

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
