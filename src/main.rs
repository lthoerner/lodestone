mod args;
mod client;
mod models;
mod server;

use clap::Parser;

use args::*;
use client::Client;

#[tokio::main]
async fn main() {
    let args = LodestoneArgs::parse();
    match args.subcommand {
        LodestoneSubcommand::Server => server::init().await.unwrap(),
        LodestoneSubcommand::Client(ClientCommand { username }) => {
            let mut client = Client::new(username);
            client.connect().unwrap();
            client.send_message("Test".to_owned()).unwrap()
        }
    }
}
