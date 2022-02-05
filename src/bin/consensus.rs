use ziggy::zigzag;
use zigzag::ziggy_blockchain_client::ZiggyBlockchainClient;

use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()>
{
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    .connect()
    .await?;

    let mut client = ZiggyBlockchainClient::new(channel);

    let response = client.get_chain(tonic::Request::new(())).await?.into_inner();
    dbg!(response);

    Ok(())
}
