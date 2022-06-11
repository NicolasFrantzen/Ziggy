use crate::block::Block;
use crate::transaction::Transaction;

use sha2::{Sha256, Digest};

use std::mem;
use std::time::SystemTime;


#[derive(Clone, Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
}


impl Blockchain
{
    pub fn new() -> Self
    {
        let new_chain = vec![
            Block::new(0, 0, Sha256::new(), Vec::new())
        ];

        Self {
            chain: new_chain,
            pending_transactions: Vec::new(),
        }
    }

    pub fn chain(&self) -> &Vec<Block> { &self.chain }

    pub fn create_block(&mut self, nonce: u64, previous_hash: Sha256) -> &Block
    {
        let index = self.chain.len() as u64;
        let transactions = mem::take(&mut self.pending_transactions);

        let new_block = Block::new(
            index,
            nonce,
            previous_hash,
            transactions
        );

        self.chain.push(new_block);

        self.get_last_block()
    }

    pub fn new_transaction(&mut self, sender: &str, recipient: &str, amount: f64)
    {
        if amount > 0.0
        {
            self.pending_transactions.push(
                Transaction::new(
                    String::from(sender),
                    String::from(recipient),
                    amount,
                    SystemTime::now()
                )
            )
        }

        dbg!(&self.pending_transactions);
    }

    pub fn get_last_block(&mut self) -> &Block
    {
        self.chain.last().expect("No elements in blockchain.")
    }

    pub fn proof_of_work(last_nonce: u64) -> u64
    {
        let mut nonce = 0;
        loop
        {
            if Blockchain::validate_proof(last_nonce, nonce)
            {
                return nonce
            }
            nonce += 1;
        }
    }

    pub fn validate_proof(last_nonce: u64, nonce: u64) -> bool
    {
        let mut hash = Sha256::new();
        hash.update(last_nonce.to_le_bytes());
        hash.update(nonce.to_le_bytes());

        let digested = hash.finalize();
        dbg!(format!("{:X}", digested));

        digested[digested.len() - 2..] == [0x00, 0x00]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_of_work()
    {
        assert!(Blockchain::validate_proof(0, 2336));
        assert!(Blockchain::validate_proof(1, 45));
        assert!(Blockchain::validate_proof(2, 32976));
    }


    #[test]
    fn test_new_transactions()
    {
        let mut blockchain = Blockchain::new();
        blockchain.new_transaction("test1", "test2", 1.0);
        blockchain.new_transaction("test2", "test1", 2.0);
        blockchain.new_transaction("test2", "test1", -2.0);

        assert!(blockchain.pending_transactions.len() == 2);
    }


    #[test]
    fn test_add_new_block()
    {
        let mut blockchain = Blockchain::new();
        blockchain.new_transaction("test1", "test2", 1.0);
        blockchain.new_transaction("test2", "test1", 2.0);

        blockchain.create_block(1234, Sha256::new());

        // Pending transactions are moved
        assert!(blockchain.pending_transactions.is_empty());
        assert!(blockchain.chain[blockchain.chain.len() - 1].transactions().len() == 2);
    }
}
