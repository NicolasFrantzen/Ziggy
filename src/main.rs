use ziggy::zigzag::ziggy_service_server::ZiggyServiceServer;
use ziggy::ziggy::Ziggy;

use tonic::transport::Server;
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()>
{
    let addr = "[::1]:50051".parse().unwrap();
    let service = Ziggy::new();
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(ZiggyServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
