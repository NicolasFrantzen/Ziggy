mod blockchain;
use blockchain::*;

tonic::include_proto!("ziggy");

fn mine_new_block(blockchain: &mut Blockchain)
{
    let last_block = blockchain.get_last_block();

    let proof = Blockchain::proof_of_work(last_block.get_proof());
    let hash = blockchain.hash();
    blockchain.create_block(proof, hash);
}

fn main() {
    let mut block = Blockchain::new();

    mine_new_block(&mut block);

    let result = Blockchain::proof_of_work(0);
    println!("{}", result);
}
