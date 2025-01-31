use sha2::{Digest, Sha256};

use crate::{
    utils::{decode_varint, encode_varint},
    BitcoinError, Result,
};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
    pub locktime: u32,
}

impl Transaction {
    pub fn deserialize(payload: &[u8]) -> Result<Self> {
        if payload.len() < 4 {
            return Err(BitcoinError::InvalidPayload(
                "Insufficient bytes for transaction".to_string(),
            ));
        }
        let version = u32::from_le_bytes(payload[0..4].try_into().unwrap());
        let (num_inputs, offset) = decode_varint(&payload[4..])?;
        let mut offset = offset + 4;
        let mut inputs = Vec::with_capacity(num_inputs as usize);
        for _ in 0..num_inputs {
            let input = TransactionInput::deserialize(&payload[offset..])?;
            offset += input.size();
            inputs.push(input);
        }
        let (num_outputs, offset_outputs) = decode_varint(&payload[offset..])?;
        offset += offset_outputs;
        let mut outputs = Vec::with_capacity(num_outputs as usize);
        for _ in 0..num_outputs {
            let output = TransactionOutput::deserialize(&payload[offset..])?;
            offset += output.size();
            outputs.push(output);
        }
        let locktime = u32::from_le_bytes(payload[offset..offset + 4].try_into().unwrap());
        Ok(Self {
            version,
            inputs,
            outputs,
            locktime,
        })
    }

    pub fn txid(&self) -> [u8; 32] {
        let mut txid: [u8; 32] = Sha256::digest(Sha256::digest(&self.serialize())).into();
        txid.reverse();
        txid
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&self.version.to_le_bytes());
        payload.extend_from_slice(&encode_varint(self.inputs.len() as u64));
        for input in &self.inputs {
            payload.extend_from_slice(&input.previous_output.hash);
            payload.extend_from_slice(&input.previous_output.index.to_le_bytes());
            payload.extend_from_slice(&encode_varint(input.script_sig.len() as u64));
            payload.extend_from_slice(&input.script_sig);
            payload.extend_from_slice(&input.sequence.to_le_bytes());
        }
        payload.extend_from_slice(&encode_varint(self.outputs.len() as u64));
        for output in &self.outputs {
            payload.extend_from_slice(&output.value.to_le_bytes());
            payload.extend_from_slice(&encode_varint(output.script_pub_key.len() as u64));
            payload.extend_from_slice(&output.script_pub_key);
        }
        payload.extend_from_slice(&self.locktime.to_le_bytes());
        payload
    }

    pub fn size(&self) -> usize {
        self.serialize().len()
    }
}
#[derive(Debug, Clone)]
pub struct TransactionInput {
    pub previous_output: OutPoint,
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

impl TransactionInput {
    pub fn deserialize(payload: &[u8]) -> Result<Self> {
        if payload.len() < 36 {
            return Err(BitcoinError::InvalidPayload(
                "Insufficient bytes for transaction input".to_string(),
            ));
        }
        let previous_output = OutPoint::deserialize(&payload[0..36])?;
        let (script_sig_len, offset) = decode_varint(&payload[36..])?;
        let offset = 36 + offset;
        let script_sig = payload[offset..offset + script_sig_len as usize].to_vec();
        let offset = offset + script_sig_len as usize;
        let sequence = &payload[offset..offset + 4];
        let sequence = u32::from_le_bytes(sequence.try_into().map_err(|_| {
            BitcoinError::InvalidPayload("Invalid sequence in transaction input".to_string())
        })?);
        let sequence = sequence as u32;
        Ok(Self {
            previous_output,
            script_sig,
            sequence,
        })
    }

    pub fn size(&self) -> usize {
        36 + encode_varint(self.script_sig.len() as u64).len()
            + self.script_sig.len()
            + self.sequence.to_le_bytes().len()
    }
}
#[derive(Debug, Clone)]
pub struct TransactionOutput {
    pub value: u64,
    pub script_pub_key: Vec<u8>,
}

impl TransactionOutput {
    pub fn deserialize(payload: &[u8]) -> Result<Self> {
        if payload.len() < 8 {
            return Err(BitcoinError::InvalidPayload(
                "Insufficient bytes for transaction output".to_string(),
            ));
        }
        let value = u64::from_le_bytes(payload[0..8].try_into().unwrap());
        let (script_pub_key_len, offset) = decode_varint(&payload[8..])?;
        let offset = offset + 8;
        let script_pub_key = payload[offset..offset + script_pub_key_len as usize].to_vec();
        Ok(Self {
            value,
            script_pub_key,
        })
    }

    pub fn size(&self) -> usize {
        self.value.to_le_bytes().len()
            + encode_varint(self.script_pub_key.len() as u64).len()
            + self.script_pub_key.len()
    }
}
#[derive(Debug, Clone)]
pub struct OutPoint {
    pub hash: [u8; 32],
    pub index: u32,
}

impl OutPoint {
    pub fn deserialize(payload: &[u8]) -> Result<Self> {
        if payload.len() < 36 {
            return Err(BitcoinError::InvalidPayload(
                "Insufficient bytes for outpoint".to_string(),
            ));
        }
        let hash = payload[0..32]
            .try_into()
            .map_err(|_| BitcoinError::InvalidPayload("Invalid outpoint hash".to_string()))?;
        let index = u32::from_le_bytes(
            payload[32..36]
                .try_into()
                .map_err(|_| BitcoinError::InvalidPayload("Invalid outpoint index".to_string()))?,
        );
        Ok(Self { hash, index })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_deserialize() {
        let payload_transaction = hex::decode("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0102ffffffff0100f2052a01000000434104d46c4968bde02899d2aa0963367c7a6ce34eec332b32e42e5f3407e052d64ac625da6f0718e7b302140434bd725706957c092db53805b821a85b23a7ac61725bac00000000").unwrap();

        let transaction = Transaction::deserialize(&payload_transaction).unwrap();

        assert_eq!(payload_transaction.len(), transaction.size());

        assert_eq!(transaction.version, 1);
        assert_eq!(transaction.locktime, 0);

        assert_eq!(transaction.inputs.len(), 1);
        let input = &transaction.inputs[0];
        assert_eq!(input.previous_output.hash, [0; 32]);
        assert_eq!(input.previous_output.index, 0xFFFFFFFF);
        assert_eq!(input.script_sig, hex::decode("04ffff001d0102").unwrap());
        assert_eq!(input.sequence, 0xFFFFFFFF);

        assert_eq!(transaction.outputs.len(), 1);
        let output = &transaction.outputs[0];
        assert_eq!(output.value, 5000000000);
        assert_eq!(
             output.script_pub_key,
             hex::decode("4104d46c4968bde02899d2aa0963367c7a6ce34eec332b32e42e5f3407e052d64ac625da6f0718e7b302140434bd725706957c092db53805b821a85b23a7ac61725bac").unwrap()
         );

        let serialized_transaction = transaction.serialize();
        assert_eq!(payload_transaction, serialized_transaction);
    }

    #[test]
    fn test_transaction_serialize() {
        let transaction_hex_test = "01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0102ffffffff0100f2052a01000000434104d46c4968bde02899d2aa0963367c7a6ce34eec332b32e42e5f3407e052d64ac625da6f0718e7b302140434bd725706957c092db53805b821a85b23a7ac61725bac00000000";
        let payload_transaction = hex::decode(transaction_hex_test).unwrap();
        let transaction = Transaction::deserialize(&payload_transaction).unwrap();
        let transaction_hex = hex::encode(transaction.serialize());

        assert_eq!(transaction_hex_test, transaction_hex);
    }
}
