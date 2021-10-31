use tonic::{transport::Server, Request, Response, Status};

use zigzag::ziggy_blockchain_server::{ZiggyBlockchain, ZiggyBlockchainServer};
use zigzag::{MineResponse};
mod zigzag;

use blockchain::{Blockchain};
mod blockchain;

fn mine_new_block(blockchain: &mut Blockchain)
{
    let last_block = blockchain.get_last_block();

    let proof = Blockchain::proof_of_work(last_block.get_proof());
    let hash = blockchain.hash();
    blockchain.create_block(proof, hash);
}

#[derive(Default)]
pub struct MyZiggyBlockchain {}

#[tonic::async_trait]
impl ZiggyBlockchain for MyZiggyBlockchain
{
    async fn mine(&self, _request: Request<()>) -> Result<Response<MineResponse>, Status>
    {
        Ok(Response::new(MineResponse{
             index: 0,
             time: 0,
             proof: 0,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let addr = "[::1]:50051".parse().unwrap();
    let service = MyZiggyBlockchain::default();
    println!("Server listening on {}", addr);

    // Blockchain
    let mut block = Blockchain::new();
    mine_new_block(&mut block);

    let result = Blockchain::proof_of_work(0);
    println!("{}", result);
    // Blockchain end

    Server::builder()
        .add_service(ZiggyBlockchainServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}