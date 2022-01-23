use std::sync::Mutex;

use tonic::{transport::Server, Request, Response, Status};
use anyhow::Result;

use ziggy::zigzag::{
    ziggy_blockchain_server::ZiggyBlockchain, ziggy_blockchain_server::ZiggyBlockchainServer, MineResponse,
    NewTransactionRequest, NewTransactionResponse, GetChainResponse, Block as GrpcBlock,
    Blockchain as GrpcBlockchain,
};

use ziggy::blockchain::{Blockchain, Block};

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
        let nonce = Blockchain::proof_of_work(last_block.nonce());
        let hash = chain.hash();

        chain.create_block(nonce, hash)
    }

    fn add_new_transaction(&self, sender: &str, recipient: &str, amount: f64)
    {
        let mut chain = self.blockchain.lock().unwrap();
        chain.new_transaction(sender, recipient, amount);
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

        Ok(Response::new(MineResponse {
            block: Some(GrpcBlock::from(new_block))
        }))
    }

    async fn new_transaction(&self, request: Request<NewTransactionRequest>) -> Result<Response<NewTransactionResponse>, Status>
    {
        println!("New transaction request received.");

        let request = request.get_ref();
        match &request.transaction
        {
            Some(transaction) => self.add_new_transaction(&transaction.sender, &transaction.recipient, transaction.amount),
            None => ()
        }

        dbg!(request);

        Ok(Response::new(NewTransactionResponse{}))
    }

    async fn get_chain(&self, _request: Request<()>) -> Result<Response<GetChainResponse>, Status>
    {
        let blocks = vec![GrpcBlock {index: 0, time: 0, nonce: 0, previous_hash: String::from("hej")}];

        Ok(Response::new(GetChainResponse {blockchain: Some(GrpcBlockchain{blocks})}))
    }
}


#[tokio::main]
async fn main() -> Result<()>
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
