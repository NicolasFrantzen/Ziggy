use std::mem;
use std::time::SystemTime;
use sha2::{Sha256, Digest};


#[derive(Clone, Debug)]
pub struct Block
{
    index: u64,
    time: SystemTime,
    nonce: u64,
    previous_hash: Sha256,
    transactions: Vec<Transaction>,
}


impl Block
{
    pub fn index(&self) -> u64 { self.index }
    pub fn nonce(&self) -> u64 { self.nonce }

    pub fn time(&self) -> u128
    {
        self.time.duration_since(SystemTime::UNIX_EPOCH).expect("").as_millis()
    }

    pub fn previous_hash(&self) -> String
    {
        let digested = self.previous_hash.clone().finalize();

        format!("{:X}", digested)
    }

    pub fn transactions(&self) -> &Vec<Transaction> { &self.transactions }
}


#[derive(Clone, Debug)]
pub struct Transaction
{
    sender: String,
    recipient: String,
    amount: f64,
    time: SystemTime,
}


impl Transaction
{
    pub fn sender(&self) -> &str { &self.sender }

    pub fn recipient(&self) -> &str { &self.recipient }

    pub fn amount(&self) -> f64 { self.amount }

    pub fn time(&self) -> u128
    {
        self.time.duration_since(SystemTime::UNIX_EPOCH).expect("").as_millis()
    }

    pub fn hash(&mut self) -> Sha256
    {
        let mut hash = Sha256::new();
        hash.update(&self.sender);
        hash.update(&self.recipient);
        hash.update(self.amount.to_le_bytes());
        hash.update(self.time().to_le_bytes());

        hash
    }
}


#[derive(Clone, Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
}


impl Blockchain
{
    pub fn new() -> Blockchain {
        let new_chain = vec![Block {
            index: 0,
            time: SystemTime::now(),
            nonce: 0,
            previous_hash: Sha256::new(),
            transactions: Vec::new(),
        }];

        Blockchain {
            chain: new_chain,
            pending_transactions: Vec::new(),
        }
    }

    pub fn chain(&self) -> &Vec<Block> { &self.chain }

    pub fn create_block(&mut self, nonce: u64, previous_hash: Sha256) -> &Block
    {
        let new_block: Block = Block {
            index: self.chain.len() as u64,
            time: SystemTime::now(),
            nonce,
            previous_hash,
            transactions: mem::take(&mut self.pending_transactions),
        };

        self.chain.push(new_block.clone());

        self.get_last_block()
    }


    pub fn new_transaction(&mut self, sender: &str, recipient: &str, amount: f64)
    {
        if amount > 0.0
        {
            self.pending_transactions.push(Transaction {
                    sender: String::from(sender),
                    recipient: String::from(recipient),
                    amount,
                    time: SystemTime::now(),
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
        hash.update(block.time().to_le_bytes());
        hash.update(block.nonce.to_le_bytes());
        hash.update(block.previous_hash.clone().finalize());
        //hash.update(block.transactions);

        hash
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
