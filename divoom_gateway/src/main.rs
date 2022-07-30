mod server;

use crate::server::*;
use clap::Parser;
use divoom::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliOptions {
    #[clap(
        short,
        long,
        help = "Server address. E.g. 127.0.0.0",
        value_parser,
        default_value = "127.0.0.1"
    )]
    server: String,

    #[clap(
        short,
        long,
        help = "Server port. E.g. 127.0.0.0",
        value_parser,
        default_value_t = 20821
    )]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let args = CliOptions::parse();
    let api_server = ApiServer::new(args.server, args.port);
    api_server.start().await
}
