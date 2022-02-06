use crate::zigzag::{
    Block as GrpcBlock, Blockchain as GrpcBlockchain, Transaction as GrpcTransaction
};
use crate::blockchain::{Block, Blockchain, Transaction};


impl From<&Blockchain> for GrpcBlockchain
{
    fn from(blockchain: &Blockchain) -> GrpcBlockchain
    {
        let blocks: Vec<_> = blockchain.chain()
            .iter()
            .map(|c| GrpcBlock::from(c))
            .collect();

        GrpcBlockchain { blocks }
    }
}


impl From<&Block> for GrpcBlock
{
    fn from(block: &Block) -> GrpcBlock
    {
        let transactions: Vec<_> = block.transactions()
            .iter()
            .map(|t| GrpcTransaction::from(t))
            .collect();

        GrpcBlock {
            index: block.index(),
            time: block.time() as u64,
            nonce: block.nonce(),
            previous_hash: block.previous_hash(),
            transactions,
        }
    }
}


impl From<&Transaction> for GrpcTransaction
{
    fn from(transaction: &Transaction) -> GrpcTransaction
    {
        GrpcTransaction {
            sender: String::from(transaction.sender()),
            recipient: String::from(transaction.recipient()),
            amount: transaction.amount(),
            time: Some(transaction.time() as u64),
        }
    }
}
