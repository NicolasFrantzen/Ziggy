#![allow(dead_code)]

use std::mem;
use std::time::{SystemTime};
use sha2::{Sha256, Digest};
//use hex_literal::hex;

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

    pub fn get_proof(&self) -> u64
    {
        self.proof
    }
}

struct Transaction
{
    sender: [char; 10],
    recipient: [char; 10],
    amount: f64,
}

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

    pub fn new_transaction(&mut self, sender: [char; 10], recipient: [char; 10], amount: f64)
    {
        self.pending_transactions.push(
            Transaction{
                sender: sender,
                recipient: recipient,
                amount: amount
            }
        )
    }

    pub fn get_last_block(&mut self) -> &Block
    {
        self.chain.last().expect("No elements in blockchain.")
    }

    pub fn hash(&mut self) -> Sha256
    {
        let block = self.get_last_block();

        let mut hash = Sha256::new();
        hash.update(block.index.to_le_bytes());
        hash.update(block.get_time().to_le_bytes());
        hash.update(block.proof.to_le_bytes());
        //hash.update(self.get_last_block().previous_hash.finalize());

        hash
    }

    pub fn proof_of_work(last_proof: u64) -> u64
    {
        let mut proof = 0;
        loop
        {
            if Blockchain::validate_proof(last_proof, proof)
            {
                return proof
            }
            proof += 1;
        }
    }

    pub fn validate_proof(last_proof: u64, proof: u64) -> bool
    {
        let mut hash = Sha256::new();
        hash.update(last_proof.to_le_bytes());
        hash.update(proof.to_le_bytes());
        let digested = hash.finalize();

        // TODO remove
        {
            let test: String = format!("{:X}", digested);
            println!("Validating: {}", test);
        }

        digested[digested.len() - 2..] == [0x00, 0x00]
    }
}
