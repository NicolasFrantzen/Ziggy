use ziggy::zigzag;
use zigzag::ziggy_service_client::ZiggyServiceClient;

use anyhow::Result;


#[tokio::main]
async fn main() -> Result<()>
{
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let mut client = ZiggyServiceClient::new(channel);

    let response = client.mine(tonic::Request::new(())).await?.into_inner();
    dbg!(response);

    Ok(())
}
