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
        let block_header_serialized = Arc::new(self.block.block_header.serialize());

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

                let mut block_header = (*block_header_serialized).clone();
                let mut hasher = Sha256::new();

                for nonce in start_nonce..=end_nonce {
                    if found.load(Ordering::Relaxed) {
                        break;
                    }

                    block_header[76..80].copy_from_slice(&nonce.to_le_bytes());

                    hasher.update(&block_header);
                    let first_hash = hasher.finalize_reset();
                    hasher.update(&first_hash);
                    let hash: [u8; 32] = hasher.finalize_reset().into();

                    if hash < target {
                        found.store(true, Ordering::Relaxed);

                        let mut mined_block = self.block.clone();
                        mined_block.block_header.nonce = nonce;
                        return Some(mined_block);
                    }
                }
                None
            });

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{merkle_root, BlockHeader, Transaction};

    use super::*;

    #[test]
    fn test_mine_block_170() {
        let mainet_bits = 0x1d00ffff;
        let timestamp = 0x496ab951;
        let coinbase_bytes = hex::decode("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0102ffffffff0100f2052a01000000434104d46c4968bde02899d2aa0963367c7a6ce34eec332b32e42e5f3407e052d64ac625da6f0718e7b302140434bd725706957c092db53805b821a85b23a7ac61725bac00000000").unwrap();
        let coinbase = Transaction::deserialize(&coinbase_bytes).unwrap();
        let merkle_root_hash = merkle_root::MerkleRoot::calculate(&[&coinbase.txid()]);
        let block_header = BlockHeader {
            bits: mainet_bits,
            nonce: 0,
            timestamp,
            version: 1,
            previous_block_hash: hex::decode(
                "000000002a22cfee1f2c846adbd12b3e183d4f97683f85dad08a79780a84bd55",
            )
            .unwrap()
            .try_into()
            .unwrap(),
            merkle_root_hash,
        };

        let block = Block {
            block_header,
            transactions: vec![coinbase],
        };

        let miner = Miner::new(block);
        let block = miner.mine().unwrap();

        println!("{:?}", block.block_header.nonce);
    }
}
