use clap::{Arg,App};

use zigzag::ziggy_blockchain_client::{ZiggyBlockchainClient};
use zigzag::{NewTransactionRequest};
mod zigzag;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args = App::new("your-app-name")
    .args(&[
        Arg::new("sender")
            .about("Sender of the transaction")
            .takes_value(true)
            .required(true),
        Arg::new("recipient")
            .about("Recipient of the transaction")
            .required(true),
        Arg::new("amount")
            .about("Amount of Ziggy to send")
            .takes_value(true)
            .required(true),

    ]).get_matches();

    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    .connect()
    .await?;

    let mut client = ZiggyBlockchainClient::new(channel);

    let response = client.new_transaction(tonic::Request::new(
        NewTransactionRequest{
            sender: String::from(args.value_of("sender").unwrap()),
            recipient: String::from(args.value_of("recipient").unwrap()),
            amount: args.value_of_t("amount").unwrap_or(0.0)
        }
    )).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}
