use crate::{BitcoinError, Result};

#[derive(Debug)]
pub struct BlockHeader {
    pub version: u32,
    pub previous_block_hash: [u8; 32],
    pub merkle_root_hash: [u8; 32],
    pub timestamp: u32,
    pub bits: u32,
    pub nonce: u32,
}

impl BlockHeader {
    pub fn deserialize(payload: &[u8]) -> Result<Self> {
        if payload.len() != 80 {
            return Err(BitcoinError::InvalidPayload(
                "Invalid block header length".to_string(),
            ));
        }
        let version = u32::from_le_bytes(
            payload[0..4]
                .try_into()
                .map_err(|_| BitcoinError::InvalidPayload("Invalid version length".to_string()))?,
        );
        let previous_block_hash = payload[4..36].try_into().map_err(|_| {
            BitcoinError::InvalidPayload("Invalid previous block hash length".to_string())
        })?;
        let merkle_root_hash = payload[36..68].try_into().map_err(|_| {
            BitcoinError::InvalidPayload("Invalid merkle root hash length".to_string())
        })?;
        let timestamp =
            u32::from_le_bytes(payload[68..72].try_into().map_err(|_| {
                BitcoinError::InvalidPayload("Invalid timestamp length".to_string())
            })?);
        let bits = u32::from_le_bytes(
            payload[72..76]
                .try_into()
                .map_err(|_| BitcoinError::InvalidPayload("Invalid bits length".to_string()))?,
        );
        let nonce = u32::from_le_bytes(
            payload[76..80]
                .try_into()
                .map_err(|_| BitcoinError::InvalidPayload("Invalid nonce length".to_string()))?,
        );

        return Ok(Self {
            version,
            previous_block_hash,
            merkle_root_hash,
            timestamp,
            bits,
            nonce,
        });
    }
}
