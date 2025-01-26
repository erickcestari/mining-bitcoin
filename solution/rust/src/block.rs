use crate::{block_header::BlockHeader, transaction::Transaction, utils, BitcoinError, Result};

#[derive(Debug)]
pub struct Block {
    pub block_header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn deserialize(payload: &[u8]) -> Result<Self> {
        if payload.len() < 80 {
            return Err(BitcoinError::InvalidPayload(
                "Invalid block length".to_string(),
            ));
        }
        let block_header = BlockHeader::deserialize(&payload[0..80])?;
        let (transaction_count, offset_count) =
            utils::decode_varint(payload[80..].try_into().unwrap()).map_err(|_| {
                BitcoinError::InvalidPayload("Invalid transaction count".to_string())
            })?;

        let mut transactions = Vec::new();
        let mut offset = 80 + offset_count;

        while transaction_count > transactions.len() as u64 {
            let transaction = Transaction::deserialize(&payload[offset..])?;
            offset += transaction.size();
            transactions.push(transaction);
        }
        Ok(Self {
            block_header,
            transactions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_deserialize() {
        let genesis_block_hex = "010000006fe28c0ab6f1b372c1a6a246ae63f74f931e8365e15a089c68d6190000000000982051fd1e4ba744bbbe680e1fee14677ba1a3c3540bf7b1cdb606e857233e0e61bc6649ffff001d01e362990101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0104ffffffff0100f2052a0100000043410496b538e853519c726a2c91e61ec11600ae1390813a627c66fb8be7947be63c52da7589379515d4e0a604f8141781e62294721166bf621e73a82cbf2342c858eeac00000000";
        let payload_block = hex::decode(genesis_block_hex).unwrap();
        let transaction_hex = &genesis_block_hex[162..];
        let block = Block::deserialize(&payload_block).unwrap();

        assert_eq!(block.block_header.version, 1);
        assert_eq!(block.block_header.timestamp, 1231469665);
        assert_eq!(block.block_header.bits, 486604799);
        assert_eq!(block.block_header.nonce, 2573394689);
        assert_eq!(
            hex::encode({
                let mut hash = block.block_header.previous_block_hash.clone();
                hash.reverse();
                hash
            }),
            "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f"
        );
        assert_eq!(
            hex::encode({
                let mut merkle = block.block_header.merkle_root_hash.clone();
                merkle.reverse();
                merkle
            }),
            "0e3e2357e806b6cdb1f70b54c3a3a17b6714ee1f0e68bebb44a74b1efd512098"
        );
        assert_eq!(block.transactions.len(), 1);
        assert_eq!(block.transactions[0].serialize(), payload_block[81..]);
        assert_eq!(
            hex::encode(block.transactions[0].serialize()),
            transaction_hex
        );
    }
}
