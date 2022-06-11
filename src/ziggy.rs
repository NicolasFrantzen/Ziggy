use crate::zigzag::{
    ziggy_service_server::ZiggyService,
    MineResponse,
    NewTransactionRequest,
    NewTransactionResponse,
    GetChainResponse,
    Block as GrpcBlock,
    Blockchain as GrpcBlockchain,
    RegisterNodesRequest,
    RegisterNodesResponse,
    ResolveConflictsRequest,
    ResolveConflictsResponse,
};

use crate::blockchain::Blockchain;

use tonic::{Request, Response, Status};
use anyhow::Result;

use std::sync::Mutex;
use std::ops::Deref;
use std::net::Ipv4Addr;



pub struct Node
{
    address: Ipv4Addr,
    port: u32,
}


type Nodes = Mutex<Vec<Node>>;


pub struct Ziggy
{
    blockchain: Mutex<Blockchain>,
    nodes: Nodes,
}


impl Ziggy
{
    pub fn new() -> Self
    {
        Self {
            blockchain: Mutex::new(Blockchain::new()),
            nodes: Mutex::new(vec![]),
        }
    }

    fn mine_new_grpc_block(&self) -> GrpcBlock
    {
        let mut chain = self.blockchain.lock().unwrap();
        let last_block = chain.get_last_block();
        let nonce = Blockchain::proof_of_work(last_block.nonce());
        let hash = last_block.hash();

        let block = chain.create_block(nonce, hash);

        GrpcBlock::from(block)
    }

    fn add_new_transaction(&self, sender: &str, recipient: &str, amount: f64)
    {
        let mut chain = self.blockchain.lock().unwrap();
        chain.new_transaction(sender, recipient, amount);
    }

    fn get_grpc_blockchain(&self) -> GrpcBlockchain
    {
        let chain = self.blockchain.lock().unwrap();

        GrpcBlockchain::from(chain.deref())
    }
}


#[tonic::async_trait]
impl ZiggyService for Ziggy
{
    async fn mine(&self, _request: Request<()>) -> Result<Response<MineResponse>, Status>
    {
        println!("Mining request received.");

        let new_block = self.mine_new_grpc_block();
        //dbg!(&new_block);

        Ok(Response::new(MineResponse {
            block: Some(new_block)
        }))
    }

    async fn new_transaction(&self, request: Request<NewTransactionRequest>) -> Result<Response<NewTransactionResponse>, Status>
    {
        println!("New transaction request received.");

        let request = request.get_ref();
        //dbg!(request);

        if let Some(transaction) = &request.transaction
        {
            self.add_new_transaction(&transaction.sender, &transaction.recipient, transaction.amount);
        }

        Ok(Response::new(NewTransactionResponse{}))
    }

    async fn get_chain(&self, _request: Request<()>) -> Result<Response<GetChainResponse>, Status>
    {
        Ok(Response::new(GetChainResponse {
            blockchain: Some(self.get_grpc_blockchain()),
        }))
    }

    async fn register_nodes(&self, _request: Request<RegisterNodesRequest>) -> Result<Response<RegisterNodesResponse>, Status>
    {
        Ok(Response::new(RegisterNodesResponse { }))
    }

    async fn resolve_conflicts(&self, _request: Request<ResolveConflictsRequest>) -> Result<Response<ResolveConflictsResponse>, Status>
    {
        /*Ok(Response::new(GetChainResponse {
            blockchain: Some(self.get_grpc_blockchain()),
        }))*/
        todo!()
    }
}
