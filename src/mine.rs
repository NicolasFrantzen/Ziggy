use zigzag::ziggy_blockchain_client::{ZiggyBlockchainClient};
mod zigzag;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    .connect()
    .await?;

    let mut client = ZiggyBlockchainClient::new(channel);

    let response = client.mine(tonic::Request::new(())).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}
