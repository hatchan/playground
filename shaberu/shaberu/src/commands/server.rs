use anyhow::{Context, Result};
use clap::Args;
use tokio::net::TcpListener;
use tracing::info;

#[derive(Args)]
pub(crate) struct Arguments {
    #[clap(long, short, env, default_value = "127.0.0.1:7676")]
    pub listen_address: String,
}

pub(crate) async fn handle_command(args: Arguments) -> Result<()> {
    let listener = TcpListener::bind(&args.listen_address)
        .await
        .with_context(|| format!("Failed to bind to address: {}", args.listen_address))?;

    let listen_address = listener
        .local_addr()
        .context("unable to retrieve local address for listener")?;
    info!(?listen_address, "Starting shaberu server");

    let service = create_server().context("unable to create axum router")?;
    axum::serve(listener, service)
        .await
        .context("unable to start axum server")?;

    Ok(())
}

fn create_server() -> Result<axum::Router> {
    todo!()
}
