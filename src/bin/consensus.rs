use clap::{Arg, App, AppSettings};
use anyhow::Result;

use tonic::transport::Channel;
use ziggy::zigzag;
use zigzag::RegisterNodesRequest;
use zigzag::register_nodes_request::Node as GrpcNode;
use zigzag::ziggy_service_client::ZiggyServiceClient;


async fn create_client() -> Result<ZiggyServiceClient<Channel>>
{
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    Ok(ZiggyServiceClient::new(channel))
}


/// Register one node
async fn register_node(address: &str, port: u32) -> Result<()>
{
    let mut client = create_client().await?;

    let new_node = GrpcNode {
        address: String::from(address),
        port
    };
    let response = client.register_nodes(tonic::Request::new(
        RegisterNodesRequest {
            nodes: vec![new_node],
         }
    )).await?.into_inner();

    dbg!(response);

    Ok(())
}


/// Get full chain
async fn get_chain() -> Result<()>
{
    let mut client = create_client().await?;

    let response = client.get_chain(tonic::Request::new(())).await?.into_inner();
    dbg!(response);

    Ok(())
}


#[tokio::main]
async fn main() -> Result<()>
{
    let args = App::new("consensus")
        .subcommand(App::new("getchain"))
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

    match args.subcommand()
    {
        Some(("getchain", _)) =>
            get_chain().await?,

        Some(("register", sub_matches)) =>
            register_node(
                sub_matches.value_of("address").unwrap(), // TODO: validate
                sub_matches.value_of_t("port").unwrap_or(50051), // TODO: validate
            ).await?,
        _ => (),
    }

    Ok(())
}
