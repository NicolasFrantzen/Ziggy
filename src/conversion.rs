use ziggy::zigzag::{
    Block as GrpcBlock, Blockchain as GrpcBlockchain,
};
use crate::blockchain::{Block, Blockchain};


impl From<Block> for GrpcBlock
{
    fn from(block: Block) -> GrpcBlock
    {
        GrpcBlock {
            index: block.get_index(),
            time: block.get_time() as u64,
            proof: block.get_proof(),
            previous_hash: block.get_previous_hash(),
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
