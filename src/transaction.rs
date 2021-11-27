use zigzag::ziggy_blockchain_client::{ZiggyBlockchainClient};
use zigzag::{NewTransactionRequest};
mod zigzag;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    .connect()
    .await?;

    let mut client = ZiggyBlockchainClient::new(channel);

    let response = client.new_transaction(tonic::Request::new(
        NewTransactionRequest{
            sender: "user1".to_string(),
            recipient: "user2".to_string(),
            amount: 1.0
        }
    )).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}
