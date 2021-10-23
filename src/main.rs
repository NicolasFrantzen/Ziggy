mod blockchain;
use blockchain::*;
use sha2::{Sha256, Digest};

fn mine_new_block(block: &mut Blockchain)
{
    block.create_block(0, Sha256::new());
}


fn main() {
    let mut block = Blockchain::new();

    mine_new_block(&mut block);
}
