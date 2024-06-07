use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: DateTime<Utc>,
    pub previous_hash: String,
    pub hash: String,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp = Utc::now();
        let mut nonce = 0;
        let mut hash = Block::calculate_hash(index, timestamp, &previous_hash, &data, nonce);
        while &hash[..4] != "0000" {
            nonce += 1;
            hash = Block::calculate_hash(index, timestamp, &previous_hash, &data, nonce);
        }
        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            data,
            nonce,
        }
    }

    pub fn calculate_hash(
        index: u32,
        timestamp: DateTime<Utc>,
        previous_hash: &str,
        data: &str,
        nonce: u64,
    ) -> String {
        let input = format!("{}{}{}{}{}", index, timestamp, previous_hash, data, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
    pub fn genesis_block() -> Block {
        Block::new(0, String::from("0"), String::from("Genesis Block"))
    }
}
