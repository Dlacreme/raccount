use super::super::transaction;
use super::error;
use super::pow;
use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use num_bigint::BigUint;
use num_traits::One;

const HASH_BYTE_SIZE: usize = 32;
const DIFFICULTY: usize = 5;
const MAX_NONCE: u64 = 1_000_000;

pub type Sha256Hash = [u8; HASH_BYTE_SIZE];

#[derive(Debug)]
pub struct Block {
    // Headers.
    timestamp: i64,
    prev_block_hash: Sha256Hash,
    nonce: u64,

    data: transaction::Transaction,
}

impl Block {

    pub fn genesis() -> Result<Self, error::MiningError> {
        Self::new("Genesis block", Sha256Hash::default())
    }

    pub fn new(data: &str, prev_hash: Sha256Hash) -> Result<Self, error::MiningError> {
        let mut s = Self {
            prev_block_hash: prev_hash,
            data: transaction::Transaction::new(String::from(data)),
            timestamp: Utc::now().timestamp(),
            nonce: 0,
        };

        s.try_hash()
            .ok_or(error::MiningError::Iteration)
            .and_then(|n| {
                s.nonce = n;
                Ok(s)
            })
    }

    pub fn hash(&self) -> Sha256Hash {
        self.calculate_hash(self.nonce)
    }

    fn try_hash(&self) -> Option<u64> {
        // Target is used to compare the hash.
        // It is a 256bit binay with $DIFFICULTY leading zeros
        let target = BigUint::one() << (256 - DIFFICULTY);

        for nonce in 0..MAX_NONCE {
            let hash = self.calculate_hash(nonce);
            let hash_int = BigUint::from_bytes_be(&hash);

            if hash_int < target {
                return Some(nonce);
            }
        }

        None
    }

    pub fn calculate_hash(&self, nonce: u64) -> Sha256Hash {
        let mut headers = self.headers();
        headers.extend_from_slice(&pow::convert_u64_to_u8_array(nonce));

        let mut hasher = Sha256::new();
        hasher.input(&headers);
        let mut hash = Sha256Hash::default();

        hasher.result(&mut hash);

        hash
    }

    pub fn headers(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.extend(&pow::convert_u64_to_u8_array(self.timestamp as u64));
        vec.extend_from_slice(&self.prev_block_hash);

        vec
    }

    pub fn print(&self) -> &str {
        self.data.id.as_str()
    }

}
