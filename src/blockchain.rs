#![allow(dead_code)]

use std::time::{SystemTime};
use sha2::{Sha256, Digest};

pub struct Blockchain {
    chain: Vec<Block>,
    transactions: Vec<Transaction>,
}

impl Blockchain
{
    pub fn new() -> Blockchain {
        let mut initial_chain: Vec<Block> = vec![];
        initial_chain.push(
            Block{
                index: 0,
                epoch: SystemTime::now(),
                proof: 0,
                previous_hash: Sha256::new()
            }
        );

        Blockchain {
            chain: initial_chain,
            transactions: vec![]
        }
    }

    pub fn create_block(&mut self, proof: u64, previous_hash: Sha256)
    {
        let new_index: usize = self.chain.len() + 1;

        let new_block: Block = Block{
            index: new_index as u64,
            epoch: SystemTime::now(),
            proof: proof,
            previous_hash: previous_hash,
        };

        self.chain.push(new_block);
    }
}

struct Block
{
    index: u64,
    epoch: SystemTime,
    proof: u64,
    previous_hash: Sha256,
}

struct Transaction
{
    sender: [char; 10],
    recipient: [char; 10],
    amount: f64,
}
