extern crate chrono;

use sha2::{Sha256};

pub struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain
{
    pub fn new() -> Blockchain {
        Blockchain {
            chain: vec![]
        }
    }

    pub fn create_block(&mut self, proof: u64, previous_hash: Sha256)
    {
        let new_index: usize = self.chain.len() + 1;

        //let mut previous_hash = Sha256::new();
        //previous_hash.update(b"Hej");

        let new_block: Block = Block{
            index: new_index as u64,
            epoch: chrono::Utc::now(),
            proof: proof,
            previous_hash: previous_hash,
        };

        self.chain.push(new_block);
    }
}

struct Block
{
    index: u64,
    epoch: chrono::DateTime<chrono::Utc>,
    proof: u64,
    previous_hash: Sha256,
}
