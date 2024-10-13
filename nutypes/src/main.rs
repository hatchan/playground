use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Client(commands::client::Args),
    Server(commands::server::Args),
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Setup logging, tracing, etc.

    let result = match args.command {
        Command::Client(args) => commands::client::handle_command(args).await,
        Command::Server(args) => commands::server::handle_command(args).await,
    };

    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }
}
