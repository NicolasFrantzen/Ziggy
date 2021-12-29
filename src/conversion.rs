use ziggy::zigzag::{
    Block as GrpcBlock, Blockchain as GrpcBlockchain,
};
use crate::blockchain::{Block, Blockchain};


impl From<Block> for GrpcBlock
{
    fn from(block: Block) -> GrpcBlock
    {
        GrpcBlock {
            index: block.index(),
            time: block.time() as u64,
            nonce: block.nonce(),
            previous_hash: block.previous_hash(),
        }
    }
}

impl From<Blockchain> for GrpcBlockchain
{
    fn from(_blockchain: Blockchain) -> GrpcBlockchain
    {
        todo!();
    }
}
