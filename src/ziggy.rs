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
use crate::node::{Nodes, Node};

use tonic::{Request, Response, Status};
use anyhow::Result;

use std::sync::Mutex;
use std::ops::Deref;


pub struct Ziggy
{
    blockchain: Mutex<Blockchain>,
    nodes: Mutex<Nodes>,
}


impl Default for Ziggy
{
    fn default() -> Self
    {
        Self::new()
    }
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

    fn register_node(&self, new_nodes: &Nodes) -> Result<(), Status>
    {
        let mut nodes = self.nodes.lock().unwrap();

        for new_node in new_nodes.iter().cloned()
        {
            if nodes.contains(&new_node)
            {
                return Err(Status::already_exists("Node has already been registered"));
            }

            nodes.push(new_node);

            #[cfg(debug_assertions)]
            dbg!(&nodes);
        }

        Ok(())
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
        println!("Get chain request received.");

        Ok(Response::new(GetChainResponse {
            blockchain: Some(self.get_grpc_blockchain()),
        }))
    }

    async fn register_nodes(&self, request: Request<RegisterNodesRequest>) -> Result<Response<RegisterNodesResponse>, Status>
    {
        println!("Register nodes request received.");

        let request_nodes = request.into_inner().nodes
            .into_iter()
            .map(Node::try_from)
            .collect::<Result<_>>();

        if let Ok(request_nodes) = request_nodes
        {
            self.register_node(&request_nodes)?;

            Ok(Response::new(RegisterNodesResponse { }))
        }
        else
        {
            Err(Status::invalid_argument("IP address is not valid"))
        }
    }

    async fn resolve_conflicts(&self, _request: Request<ResolveConflictsRequest>) -> Result<Response<ResolveConflictsResponse>, Status>
    {
        println!("Resolve conflicts request received.");

        /*Ok(Response::new(GetChainResponse {
            blockchain: Some(self.get_grpc_blockchain()),
        }))*/
        todo!()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::zigzag::register_nodes_request::Node as GrpcNode;

    fn new_request(ip: &str, port: u32) -> tonic::Request<RegisterNodesRequest>
    {
        let new_node = GrpcNode {
            address: String::from(ip),
            port: port,
        };

        let request = tonic::Request::new(
            RegisterNodesRequest {
                nodes: vec![new_node],
                }
        );

        request
    }

    #[tokio::test]
    async fn test_register_nodes()
    {
        let service = Ziggy::new();

        // First register is ok
        let result = service.register_nodes(new_request("127.0.0.1", 50052)).await;
        assert!(result.is_ok());

        // Test already exists
        let result = service.register_nodes(new_request("127.0.0.1", 50052)).await;
        assert_eq!(result.unwrap_err().code(), tonic::Code::AlreadyExists);

    }

    #[tokio::test]
    async fn test_register_nodes_invalid_ip()
    {
        let service = Ziggy::new();

        let result = service.register_nodes(new_request("localhost", 50052)).await;
        assert_eq!(result.unwrap_err().code(), tonic::Code::InvalidArgument);
    }
}
