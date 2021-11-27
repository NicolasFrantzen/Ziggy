use std::sync::Mutex;

use tonic::{transport::Server, Request, Response, Status};

use zigzag::ziggy_blockchain_server::{ZiggyBlockchain, ZiggyBlockchainServer};
use zigzag::{MineResponse, NewTransactionRequest, NewTransactionResponse};
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

        // TODO: maybe use get_last_block ref instead
        chain.create_block(proof, hash)
    }

    fn add_new_transaction(&self, sender: String, recipient: String, amount: f64)
    {
        let mut chain = self.blockchain.lock().unwrap();
        chain.new_transaction(sender, recipient, amount)
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

    async fn new_transaction(&self, request: Request<NewTransactionRequest>) -> Result<Response<NewTransactionResponse>, Status>
    {
        println!("New transaction request received.");

        let request = request.get_ref();

        dbg!(request);

        self.add_new_transaction(request.sender.clone(), request.recipient.clone(), request.amount);

        Ok(Response::new(NewTransactionResponse{}))
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
