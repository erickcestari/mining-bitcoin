use mine_block::{BlockHeader, DIFFICULTY_TARGET, PREVIOUS_BLOCK_HASH};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let previous_block_hash = hex::decode(PREVIOUS_BLOCK_HASH)
        .unwrap()
        .try_into()
        .unwrap();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let block_header = BlockHeader {
        version: 1,
        previous_block_hash,
        merkle_root_hash: [0; 32],
        timestamp,
        bits: DIFFICULTY_TARGET,
        nonce: 0,
    };
    let transaction_payload = hex::decode(mine_block::TRANSACTION_SERIALIZED).unwrap();
    let transaction = mine_block::Transaction::deserialize(&transaction_payload).unwrap();

    let block = mine_block::Block {
        block_header,
        transactions: vec![transaction],
    };
}
