#![allow(dead_code)]

use std::mem;
use std::time::{SystemTime};
use sha2::{Sha256, Digest};

pub struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain
{
    pub fn new() -> Blockchain {
        let mut new_chain = Vec::new();
        new_chain.push(
            Block{
                index: 0,
                epoch: SystemTime::now(),
                proof: 0,
                previous_hash: Sha256::new(),
                transactions: Vec::new(),
            }
        );

        Blockchain {
            chain: new_chain,
            pending_transactions: Vec::new(),
        }
    }


    pub fn create_block(&mut self, proof: u64, previous_hash: Sha256)
    {
        let transactions = mem::take(&mut self.pending_transactions);
        assert!(self.pending_transactions.is_empty());

        let new_block: Block = Block{
            index: (self.chain.len() + 1) as u64,
            epoch: SystemTime::now(),
            proof: proof,
            previous_hash: previous_hash,
            transactions: transactions,
        };

        self.chain.push(new_block);
    }

    pub fn get_last_block(&mut self) -> &Block
    {
        self.chain.last().expect("No elements in blockchain.")
    }

    pub fn hash(block: &Block) -> Sha256
    {
        let mut hash = Sha256::new();
        hash.update(block.index.to_le_bytes());
        hash.update(block.get_time().to_le_bytes());
        hash.update(block.proof.to_le_bytes());
        //hash.update(self.get_last_block().previous_hash.finalize());

        hash
    }
}

pub struct Block
{
    index: u64,
    epoch: SystemTime,
    proof: u64,
    previous_hash: Sha256,
    transactions: Vec<Transaction>,
}

impl Block
{
    pub fn get_time(&self) -> u128
    {
        self.epoch.duration_since(SystemTime::UNIX_EPOCH).expect("").as_millis()
    }
}

struct Transaction
{
    sender: [char; 10],
    recipient: [char; 10],
    amount: f64,
}
