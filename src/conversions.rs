use crate::zigzag::{
    Block as GrpcBlock,
    Blockchain as GrpcBlockchain,
    Transaction as GrpcTransaction,
    register_nodes_request::Node as GrpcNode,
};

use crate::{
    blockchain::Blockchain,
    block::Block,
    transaction::Transaction,
    node::Node,
};

use std::net::Ipv4Addr;
use std::str::FromStr;
use anyhow::{Result, Error};


impl From<&Blockchain> for GrpcBlockchain
{
    fn from(blockchain: &Blockchain) -> GrpcBlockchain
    {
        let blocks: Vec<_> = blockchain.chain()
            .iter()
            .map(GrpcBlock::from)
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
            .map(GrpcTransaction::from)
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


impl TryFrom<GrpcNode> for Node
{
    type Error = Error;

    fn try_from(node: GrpcNode) -> Result<Node>
    {
        Ok(Node::new(Ipv4Addr::from_str(&node.address)?, node.port))
    }
}
