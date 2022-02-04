pub mod zigzag;
pub mod blockchain;
mod conversion;

use zigzag::{
    ziggy_blockchain_server::ZiggyBlockchain, MineResponse,
    NewTransactionRequest, NewTransactionResponse, GetChainResponse, Block as GrpcBlock,
    Blockchain as GrpcBlockchain,
};

use blockchain::{Blockchain, Block};

use tonic::{Request, Response, Status};
use anyhow::Result;

use std::sync::Mutex;

pub struct Ziggy
{
    blockchain: Mutex<Blockchain>
}


impl Ziggy
{
    pub fn new() -> Self
    {
        Self { blockchain: Mutex::new(Blockchain::new()) }
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
impl ZiggyBlockchain for Ziggy
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
