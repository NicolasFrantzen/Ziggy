use clap::{Arg, App, AppSettings};
use anyhow::{Result, anyhow};

use ziggy::zigzag;
use zigzag::ziggy_service_client::ZiggyServiceClient;


async fn register_node()
{

}


#[tokio::main]
async fn main() -> Result<()>
{
    let args = App::new("consensus")
        .subcommand(App::new("register")
            .arg(Arg::new("address")
                .help("Address of the node to register")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("port")
                .help("Port of the node to register")
                .takes_value(true)
                .required(true)))
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    match args.subcommand_name()
    {
        Some(args) => match args {
            "register" => println!("Register not implemented yet"),
            _ => ()
        },
        None => (),
    }


    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let mut client = ZiggyServiceClient::new(channel);

    let response = client.get_chain(tonic::Request::new(())).await?.into_inner();
    dbg!(response);

    Ok(())
}
