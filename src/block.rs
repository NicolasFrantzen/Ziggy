use crate::transaction::Transaction;

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
    pub fn new(index: u64, nonce: u64, previous_hash: Sha256, transactions: Vec<Transaction>) -> Self
    {
        Self {
            index,
            time: SystemTime::now(),
            nonce,
            previous_hash,
            transactions,
        }
    }

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

    pub fn as_bytes(&self) -> Vec<u8>
    {
        self.index().to_le_bytes()
            .iter()
            .cloned()
            .chain(self.time()
                .to_le_bytes()
                .iter()
                .cloned())
            .chain(self.nonce()
                .to_le_bytes()
                .iter()
                .cloned())
            .chain(self.previous_hash()
                .as_bytes()
                .iter()
                .cloned())
            .chain(self.transactions()
                .iter()
                .cloned()
                .flat_map(|t| t.as_bytes()))
        .collect()
    }

    pub fn hash(&self) -> Sha256
    {
        let mut hash = Sha256::new();
        hash.update(self.as_bytes());

        hash
    }
}


#[cfg(test)]
mod tests
{
    use super::*;
    use std::ops::Add;


    fn get_block_with_one_transaction() -> Block
    {
        let mut previous_hash = Sha256::new();
        previous_hash.update("hest".as_bytes());

        let transactions = vec![Transaction::new(
            String::from("bob"),
            String::from("alice"),
            1.0,
            SystemTime::UNIX_EPOCH.add(std::time::Duration::new(1234567890,0)),
        )];

        Block {
            index: 42,
            time: SystemTime::UNIX_EPOCH.add(std::time::Duration::new(1234567890,0)),
            nonce: 1337,
            previous_hash,
            transactions
        }
    }

    #[test]
    fn test_as_bytes()
    {
        let block_bytes = [
            42, 0, 0, 0, 0, 0, 0, 0, 80, 4, 251, 113, 31, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 5,
            0, 0, 0, 0, 0, 0, 66, 56, 51, 49, 66, 51, 69, 51, 51, 54, 69, 67, 66, 49, 51, 49, 69,
            66, 67, 53, 48, 49, 52, 51, 57, 51, 67, 48, 56, 52, 66, 70, 53, 68, 49, 53, 56, 48, 56,
            53, 52, 50, 49, 50, 70, 54, 52, 68, 70, 57, 69, 68, 57, 50, 50, 54, 70, 69, 69, 66, 67,
            52, 65, 69, 98, 111, 98, 97, 108, 105, 99, 101, 0, 0, 0, 0, 0, 0, 240, 63, 80, 4, 251,
            113, 31, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ];
        assert_eq!(get_block_with_one_transaction().as_bytes(), block_bytes);
    }

    #[test]
    fn test_hash()
    {
        let digested_hash = get_block_with_one_transaction().hash().finalize();

        assert_eq!(
            format!("{:X}", digested_hash),
            "5D22416F095A0F88310B1F808D68D96A42FE0AA1A19F0F05A834653C69585251"
        );
    }
}
