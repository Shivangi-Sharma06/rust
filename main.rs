use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u128,
    nonce: u64,
    prev_hash: String,
    hash: String,
    txs: Vec<String>,
}

impl Block {
    fn new(index: u64, prev_hash: String, txs: Vec<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let mut block = Block {
            index,
            timestamp,
            nonce: 0,
            prev_hash,
            hash: String::new(),
            txs,
        };

        block.hash = block.calc_hash();
        block
    }

    fn calc_hash(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.index.to_string());
        s.push_str(&self.timestamp.to_string());
        s.push_str(&self.nonce.to_string());
        s.push_str(&self.prev_hash);
        for tx in &self.txs {
            s.push('|');
            s.push_str(tx);
        }

        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        hex::encode(hasher.finalize())
    }
}

fn main() {
    // Genesis block
    let genesis = Block::new(0, "0".to_string(), vec!["Alice->Bob:10".to_string()]);
    println!("Genesis Block: {:?}", genesis);

    // Second block
    let block2 = Block::new(1, genesis.hash.clone(), vec!["Bob->Charlie:5".to_string()]);
    println!("Block 2: {:?}", block2);
}
