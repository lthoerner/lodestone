mod args;
mod client;
mod models;
mod server;

use clap::Parser;

use args::*;

#[tokio::main]
async fn main() {
    let args = LodestoneArgs::parse();
    match args.subcommand {
        LodestoneSubcommand::Server => server::init().await.unwrap(),
        LodestoneSubcommand::Client(ClientCommand { username }) => client::init(username).unwrap(),
    }
}
