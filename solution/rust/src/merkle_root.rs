use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct MerkleRoot {
    pub hashes: Vec<[u8; 32]>,
}

impl MerkleRoot {
    pub fn calculate(hashes: &[&[u8]]) -> [u8; 32] {
        if hashes.is_empty() {
            return [0u8; 32];
        }

        let mut current_level: Vec<Vec<u8>> = hashes.iter().map(|h| h.to_vec()).collect();

        while current_level.len() > 1 {
            let mut next_level = Vec::new();

            if current_level.len() % 2 == 1 {
                current_level.push(current_level.last().unwrap().clone());
            }

            for chunk in current_level.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(&chunk[0]);
                hasher.update(&chunk[1]);
                let first_hash = hasher.finalize();

                let mut hasher = Sha256::new();
                hasher.update(&first_hash);
                let double_hash = hasher.finalize();

                next_level.push(double_hash.to_vec());
            }

            current_level = next_level;
        }

        let mut result = [0u8; 32];
        result.copy_from_slice(&current_level[0]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_empty_merkle_root() {
        let result = MerkleRoot::calculate(&[]);
        assert_eq!(result, [0u8; 32]);
    }

    #[test]
    fn test_single_node() {
        let data = b"hello";
        let hash = Sha256::digest(Sha256::digest(data));
        let input = [hash.as_slice()];
        let result = MerkleRoot::calculate(&input);
        assert_eq!(result, hash.as_slice());
    }

    #[test]
    fn test_two_nodes() {
        let a = Sha256::digest(Sha256::digest(b"a"));
        let b = Sha256::digest(Sha256::digest(b"b"));
        let input = [a.as_slice(), b.as_slice()];

        let mut concat = a.to_vec();
        concat.extend_from_slice(&b);
        let first = Sha256::digest(&concat);
        let second = Sha256::digest(&first);
        let expected = second;

        let result = MerkleRoot::calculate(&input);
        assert_eq!(result[..], expected[..]);
    }

    #[test]
    fn test_three_nodes() {
        let a = Sha256::digest(Sha256::digest(b"a"));
        let b = Sha256::digest(Sha256::digest(b"b"));
        let c = Sha256::digest(Sha256::digest(b"c"));
        let input = [a.as_slice(), b.as_slice(), c.as_slice()];

        let d = c.clone();

        let ab_concat = [a.as_slice(), b.as_slice()].concat();
        let ab_first = Sha256::digest(&ab_concat);
        let ab_hash = Sha256::digest(&ab_first);

        let cd_concat = [c.as_slice(), d.as_slice()].concat();
        let cd_first = Sha256::digest(&cd_concat);
        let cd_hash = Sha256::digest(&cd_first);

        let ab_cd_concat = [ab_hash.to_vec(), cd_hash.to_vec()].concat();
        let ab_cd_first = Sha256::digest(&ab_cd_concat);
        let expected_root = Sha256::digest(&ab_cd_first);

        let result = MerkleRoot::calculate(&input);
        assert_eq!(result[..], expected_root[..]);
    }

    #[test]
    fn test_four_nodes() {
        let a = Sha256::digest(Sha256::digest(b"a"));
        let b = Sha256::digest(Sha256::digest(b"b"));
        let c = Sha256::digest(Sha256::digest(b"c"));
        let d = Sha256::digest(Sha256::digest(b"d"));
        let input = [a.as_slice(), b.as_slice(), c.as_slice(), d.as_slice()];

        let ab_concat = [a.as_slice(), b.as_slice()].concat();
        let ab_first = Sha256::digest(&ab_concat);
        let ab_hash = Sha256::digest(&ab_first);

        let cd_concat = [c.as_slice(), d.as_slice()].concat();
        let cd_first = Sha256::digest(&cd_concat);
        let cd_hash = Sha256::digest(&cd_first);

        let ab_cd_concat = [ab_hash.to_vec(), cd_hash.to_vec()].concat();
        let ab_cd_first = Sha256::digest(&ab_cd_concat);
        let expected_root = Sha256::digest(&ab_cd_first);

        let result = MerkleRoot::calculate(&input);
        assert_eq!(result[..], expected_root[..]);
    }
}
