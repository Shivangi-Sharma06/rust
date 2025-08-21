use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io;

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

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis = Block::new(0, "0".to_string(), vec!["Genesis Block".to_string()]);
        Blockchain {
            chain: vec![genesis],
        }
    }

    fn add_block(&mut self, txs: Vec<String>) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let index = self.chain.len() as u64;
        let block = Block::new(index, prev_hash, txs);
        self.chain.push(block);
    }

    fn print_chain(&self) {
        for block in &self.chain {
            println!("{:#?}", block);
        }
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    loop {
        println!("\n1. Add new block\n2. Show blockchain\n3. Exit");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("Enter transaction (e.g., Alice->Bob:10): ");
                let mut tx = String::new();
                io::stdin().read_line(&mut tx).unwrap();
                blockchain.add_block(vec![tx.trim().to_string()]);
                println!("✅ Block added!");
            }
            "2" => {
                println!("📜 Blockchain:");
                blockchain.print_chain();
            }
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}
