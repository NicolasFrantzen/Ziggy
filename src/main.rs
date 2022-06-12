use ziggy::zigzag::ziggy_service_server::ZiggyServiceServer;
use ziggy::ziggy::Ziggy;

use clap::{Arg, App};
use tonic::transport::Server;
use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()>
{
    let _args = App::new("ziggy")
        .arg(Arg::new("port")
            .help("Port to listen on")
            .required(false))
        .get_matches(); // TODO: use port

    let addr = "[::1]:50051".parse().unwrap();
    let service = Ziggy::new();
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(ZiggyServiceServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
