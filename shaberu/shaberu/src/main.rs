use clap::Parser;
use tracing::error;

pub(crate) mod commands;

#[derive(Parser)]
struct Arguments {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(clap::Subcommand)]
enum SubCommand {
    Server(commands::server::Arguments),
}

#[tokio::main]
async fn main() {
    let arguments = Arguments::parse();

    let result = match arguments.command {
        SubCommand::Server(arguments) => commands::server::handle_command(arguments).await,
    };

    if let Err(err) = result {
        error!(?err, "command failed");
    }
}
