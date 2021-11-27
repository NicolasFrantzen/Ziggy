use std::sync::Mutex;

use tonic::{transport::Server, Request, Response, Status};

use zigzag::ziggy_blockchain_server::{ZiggyBlockchain, ZiggyBlockchainServer};
use zigzag::{MineResponse};
mod zigzag;

use blockchain::{Blockchain, Block};
mod blockchain;


struct MyZiggyBlockchain
{
    blockchain: Mutex<Blockchain>
}


impl MyZiggyBlockchain
{
    pub fn new() -> MyZiggyBlockchain
    {
        MyZiggyBlockchain { blockchain: Mutex::new(Blockchain::new()) }
    }

    fn mine_new_block(&self) -> Block
    {
        let mut chain = self.blockchain.lock().unwrap();
        let last_block = chain.get_last_block();
        let proof = Blockchain::proof_of_work(last_block.get_proof());
        let hash = chain.hash();

        chain.create_block(proof, hash)
    }
}


#[tonic::async_trait]
impl ZiggyBlockchain for MyZiggyBlockchain
{
    async fn mine(&self, _request: Request<()>) -> Result<Response<MineResponse>, Status>
    {
        println!("Mining request received.");

        let new_block = self.mine_new_block();

        dbg!(&new_block);

        Ok(Response::new(MineResponse{
             index: new_block.get_index(),
             time: new_block.get_time() as u64,
             proof: new_block.get_proof(),
             previous_hash: new_block.get_previous_hash(),
        }))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let addr = "[::1]:50051".parse().unwrap();
    let service = MyZiggyBlockchain::new();
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(ZiggyBlockchainServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
