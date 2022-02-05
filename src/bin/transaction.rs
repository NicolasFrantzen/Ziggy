use clap::{Arg, App, AppSettings};
use anyhow::{Result, bail};

use ziggy::zigzag;
use zigzag::ziggy_blockchain_client::ZiggyBlockchainClient;
use zigzag::{
    NewTransactionRequest, Transaction as GrpcTransaction
};


fn check_amount(argument: &str) -> Result<()>
{
    if let Ok(amount) = argument.parse::<f64>()
    {
        if amount <= 0.0
        {
            bail!("Must be positive");
        }
    }
    else
    {
        bail!("Must be positive");
    }

    Ok(())
}


#[tokio::main]
async fn main() -> Result<()>
{
    let args = App::new("transaction")
        .subcommand(App::new("new")
            .arg(Arg::new("sender")
                .help("Sender of the transaction")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("recipient")
                .help("Recipient of the transaction")
                .required(true))
            .arg(Arg::new("amount")
                .help("Amount of Ziggy to send")
                .takes_value(true)
                .validator(check_amount)
                .required(true)))
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if let Some(args) = args.subcommand_matches("new")
    {
        let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
                        .connect()
                        .await?;

        let mut client = ZiggyBlockchainClient::new(channel);

        let response = client.new_transaction(tonic::Request::new(
            NewTransactionRequest {
                transaction: Some(GrpcTransaction {
                    sender: String::from(args.value_of("sender").unwrap()),
                    recipient: String::from(args.value_of("recipient").unwrap()),
                    amount: args.value_of_t("amount").unwrap_or(0.0),
                    time: None
                })
            }
        )).await?.into_inner();

        dbg!(response);
    }

    Ok(())
}
