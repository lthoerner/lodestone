use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct LodestoneArgs {
    /// Whether to run a new server, or connect to an existing server
    #[clap(subcommand)]
    pub subcommand: LodestoneSubcommand,
}

#[derive(Subcommand, Clone)]
pub enum LodestoneSubcommand {
    /// Launch a server for clients to connect to
    Server,
    /// Launch the client to connect to an existing server
    Client(ClientCommand),
}

#[derive(Args, Clone)]
pub struct ClientCommand {
    /// The name that will be displayed to other users
    #[arg(default_value = "anonymous")]
    pub username: String,
}
