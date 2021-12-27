use std::mem;
use std::time::SystemTime;
use sha2::{Sha256, Digest};


#[derive(Clone, Debug)]
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
    pub fn get_index(&self) -> u64 { self.index }
    pub fn get_proof(&self) -> u64 { self.proof }

    pub fn get_time(&self) -> u128
    {
        self.epoch.duration_since(SystemTime::UNIX_EPOCH).expect("").as_millis()
    }

    pub fn get_previous_hash(&self) -> String
    {
        let digested = self.previous_hash.clone().finalize();

        format!("{:X}", digested)
    }
}


#[derive(Clone, Debug)]
pub struct Transaction
{
    sender: String,
    recipient: String,
    amount: f64,
}


impl Transaction
{
    pub fn hash(&mut self) -> Sha256
    {
        todo!();
    }
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
            Block {
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


    // TODO should probably return something smaller
    // Cloning is very expensive
    pub fn create_block(&mut self, proof: u64, previous_hash: Sha256) -> Block
    {
        let new_block: Block = Block {
            index: (self.chain.len() + 1) as u64,
            epoch: SystemTime::now(),
            proof: proof,
            previous_hash: previous_hash,
            transactions: mem::take(&mut self.pending_transactions),
        };

        self.chain.push(new_block.clone());

        new_block
    }


    pub fn new_transaction(&mut self, sender: &str, recipient: &str, amount: f64)
    {
        if amount > 0.0
        {
            self.pending_transactions.push(Transaction {
                    sender: String::from(sender),
                    recipient: String::from(recipient),
                    amount
                }
            );
        }

        dbg!(&self.pending_transactions);
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
        hash.update(block.previous_hash.clone().finalize());
        //hash.update(block.transactions);

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
        dbg!(format!("{:X}", digested));

        digested[digested.len() - 2..] == [0x00, 0x00]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proof_of_work()
    {
        assert!(Blockchain::validate_proof(0, 2336));
        assert!(Blockchain::validate_proof(1, 45));
        assert!(Blockchain::validate_proof(2, 32976));
    }


    #[test]
    fn new_transactions()
    {
        let mut blockchain = Blockchain::new();
        blockchain.new_transaction("test1", "test2", 1.0);
        blockchain.new_transaction("test2", "test1", 2.0);

        assert!(blockchain.pending_transactions.len() == 2);
    }


    #[test]
    fn add_new_block()
    {
        let mut blockchain = Blockchain::new();
        blockchain.new_transaction("test1", "test2", 1.0);
        blockchain.new_transaction("test2", "test1", 2.0);

        blockchain.create_block(1234, Sha256::new());

        // Pending transactions are moved
        assert!(blockchain.pending_transactions.is_empty());
        assert!(blockchain.chain[blockchain.chain.len() - 1].transactions.len() == 2);
    }
}
