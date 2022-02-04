use ziggy::zigzag::ziggy_blockchain_server::ZiggyBlockchainServer;
use ziggy::Ziggy;

use tonic::transport::Server;
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()>
{
    let addr = "[::1]:50051".parse().unwrap();
    let service = Ziggy::new();
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(ZiggyBlockchainServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
