mod server;

use crate::server::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliOptions {
    #[clap(help = "Device address.", value_parser)]
    device_address: String,

    #[clap(
        short = 's',
        long = "server",
        help = "Server address.",
        value_parser,
        default_value = "127.0.0.1"
    )]
    server_address: String,

    #[clap(
        short = 'p',
        long = "port",
        help = "Server port.",
        value_parser,
        default_value_t = 20821
    )]
    server_port: u16,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let args = CliOptions::parse();

    let url = format!("http://{}:{}", args.server_address, args.server_port);
    println!("Starting divoom gateway on: {} for device {}.", url, args.device_address);
    println!("Please open your browser with URL: {} and happy divooming!", url);

    let api_server = ApiServer::new(args.server_address, args.server_port, args.device_address);
    api_server.start().await
}
