use mine_block::{
    Block, BlockHeader, MerkleRoot, Miner, OutPoint, Transaction, TransactionInput,
    TransactionOutput, DIFFICULTY_TARGET, PREVIOUS_BLOCK_HASH, TRANSACTION_SERIALIZED,
};
use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    let transaction_payload = hex::decode(TRANSACTION_SERIALIZED).unwrap();
    let transaction = Transaction::deserialize(&transaction_payload).unwrap();

    let coinbase_transaction = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                hash: [0; 32],
                index: 0xFFFFFFFF,
            },
            script_sig: b"erickcestari".to_vec(),
            sequence: 0xFFFFFFFF,
        }],
        outputs: vec![TransactionOutput {
            value: 5000000000,
            script_pub_key: vec![51],
        }],
        locktime: 0,
    };

    let previous_block_hash = hex::decode(PREVIOUS_BLOCK_HASH)
        .unwrap()
        .try_into()
        .unwrap();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let mut transactions_txid = Vec::new();
    let txid = transaction.txid();
    transactions_txid.push(txid.as_slice());
    let txid = coinbase_transaction.txid();
    transactions_txid.push(txid.as_slice());

    let merkle_root_hash = MerkleRoot::calculate(&transactions_txid);

    let block_header = BlockHeader {
        version: 1,
        previous_block_hash,
        merkle_root_hash,
        timestamp,
        bits: DIFFICULTY_TARGET,
        nonce: 0,
    };

    let block = Block {
        block_header,
        transactions: vec![coinbase_transaction, transaction],
    };

    let miner = Miner::new(block);
    let block_mined = miner.mine();
    match block_mined {
        Some(block) => {
            println!("valid block found {:?}", block);
        }
        None => {
            println!("No valid block found");
        }
    };
}
