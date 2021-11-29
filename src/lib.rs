extern crate chrono;
extern crate crypto_hash;
extern crate serde_json;

use chrono::Utc;
use crypto_hash::{hex_digest, Algorithm};
use serde::{Deserialize, Serialize};

pub const PREFIX: &str = "00";

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_id: String,
    pub transaction_timestamp: i64,
    pub transaction_details: String,
}

#[derive(Serialize, Deserialize)]
pub struct Block {
    pub block_number: u64,
    block_timestamp: i64,
    pub block_nonce: u64,
    pub transaction_list: Vec<Transaction>,
    previous_block_hash: String,
}

impl Block {
    pub fn genesis() -> Self {
        let transaction = Transaction {
            transaction_id: String::from("1"),
            transaction_details: String::from(
                "This is a dummy transaction as the genesis block has no transactions",
            ),
            transaction_timestamp: Utc::now().timestamp(),
        };
        Block {
            block_number: 1,
            block_timestamp: Utc::now().timestamp(),
            block_nonce: 0,
            transaction_list: vec![transaction],
            previous_block_hash: String::from("0"),
        }
    }

    pub fn serialize_block(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn generate_hash(block: &Block) -> String {
        hex_digest(Algorithm::SHA256, block.serialize_block().as_bytes())
    }

    pub fn is_block_valid(hash: &str, prefix: &str) -> bool {
        hash.starts_with(prefix)
    }

    pub fn new(transactions: Vec<Transaction>, previous_block: &Block) -> Block {
        Block {
            block_number: previous_block.block_number + 1,
            block_timestamp: Utc::now().timestamp(),
            block_nonce: 0,
            transaction_list: transactions,
            previous_block_hash: Self::generate_hash(previous_block),
        }
    }

    pub fn mine_new_block(block_candidate: &mut Block, prefix: &str) {
        while !Self::is_block_valid(&Self::generate_hash(block_candidate), prefix) {
            println!("{}", block_candidate.block_nonce);
            block_candidate.block_nonce += 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_genesis_block() {
        //create blockchain
        let p2p_bc: Vec<Block> = vec![Block::genesis()];
        assert_eq!(p2p_bc[0].block_number, 1);
        assert_eq!(
            p2p_bc[0].transaction_list[0].transaction_details,
            "This is a dummy transaction as the genesis block has no transactions"
        );
    }

    #[test]
    fn test_new_block() {
        let mut p2p_bc: Vec<Block> = vec![Block::genesis()];
        let new_txn = Transaction {
            transaction_id: String::from("1"),
            transaction_timestamp: 0,
            transaction_details: String::from("Testing a new transaction"),
        };
        let mut new_block = Block::new(vec![new_txn], &p2p_bc[p2p_bc.len() -1]);

        Block::mine_new_block(&mut new_block, &PREFIX);
        p2p_bc.push(new_block);

        assert_eq!(p2p_bc.len(), 2);
        assert_eq!(
            p2p_bc[1].transaction_list[0].transaction_details,
            "Testing a new transaction"
        )
    }
}
