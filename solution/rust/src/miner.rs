use crate::{utils, Block};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub struct Miner {
    pub block: Block,
}

impl Miner {
    pub fn new(block: Block) -> Self {
        Self { block }
    }

    pub fn mine(self) -> Option<Block> {
        let num_workers = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let target = utils::bits_to_target(self.block.block_header.bits);
        let found = Arc::new(AtomicBool::new(false));

        let block_header_serialized = self.block.block_header.serialize();

        const BATCH_SIZE: u32 = 1000;

        let result = (0..num_workers)
            .into_par_iter()
            .find_map_first(|worker_id| {
                if found.load(Ordering::Relaxed) {
                    return None;
                }

                let nonce_range = u32::MAX / num_workers as u32;
                let start_nonce = worker_id as u32 * nonce_range;
                let end_nonce = if worker_id == num_workers - 1 {
                    u32::MAX
                } else {
                    start_nonce + nonce_range - 1
                };

                let mut local_header = block_header_serialized.clone();
                let mut hasher = Sha256::new();
                let mut hash_buffer = [0u8; 32];

                let mut current_batch = 0;
                let mut nonce = start_nonce;

                while nonce <= end_nonce {
                    if current_batch == 0 && found.load(Ordering::Relaxed) {
                        break;
                    }

                    local_header[76..80].copy_from_slice(&nonce.to_le_bytes());

                    hasher.update(&local_header);
                    let first_hash = hasher.finalize_reset();

                    hasher.update(&first_hash);
                    hash_buffer.copy_from_slice(&hasher.finalize_reset());
                    hasher.reset();

                    hash_buffer.reverse();

                    if hash_buffer < target {
                        found.store(true, Ordering::Relaxed);

                        let mut mined_block = self.block.clone();
                        mined_block.block_header.nonce = nonce;
                        return Some(mined_block);
                    }

                    nonce += 1;
                    current_batch = (current_batch + 1) % BATCH_SIZE;
                }
                None
            });

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{merkle_root, BlockHeader, Transaction, TRANSACTION_SERIALIZED};

    use super::*;

    #[test]
    fn test_mine_block_170() {
        let coinbase_bytes = hex::decode("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0102ffffffff0100f2052a01000000434104d46c4968bde02899d2aa0963367c7a6ce34eec332b32e42e5f3407e052d64ac625da6f0718e7b302140434bd725706957c092db53805b821a85b23a7ac61725bac00000000").unwrap();
        let transaction_bytes = hex::decode(TRANSACTION_SERIALIZED).unwrap();
        let mainet_bits = 0x1d00ffff;
        let timestamp = 0x496ab951;

        let coinbase = Transaction::deserialize(&coinbase_bytes).unwrap();
        let transaction = Transaction::deserialize(&transaction_bytes).unwrap();

        let merkle_root_hash =
            merkle_root::MerkleRoot::calculate(&[&coinbase.txid(), &transaction.txid()]);

        let block_header = BlockHeader {
            bits: mainet_bits,
            nonce: 0,
            timestamp,
            version: 1,
            previous_block_hash: hex::decode(
                "55bd840a78798ad0da853f68974f3d183e2bd1db6a842c1feecf222a00000000",
            )
            .unwrap()
            .try_into()
            .unwrap(),
            merkle_root_hash,
        };

        let block = Block {
            block_header,
            transactions: vec![coinbase, transaction],
        };

        let miner = Miner::new(block);
        let block = miner.mine().unwrap();

        assert_eq!(block.block_header.nonce, 1889418792);
    }
}
