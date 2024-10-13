use std::process::exit;

use anyhow::{Context, Result};
use nutypes::server::create_api;
use tokio::net::TcpListener;
use tracing::{info, warn};

#[derive(Debug, clap::Args)]
pub struct Args {
    /// The address to listen on.
    #[arg(short, long, env, default_value = "127.0.0.1:6767")]
    pub listen_address: String,
}

pub async fn handle_command(args: Args) -> Result<()> {
    let api_shutdown = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for ctrl-c");

        info!("Received SIGINT, shutting down api server");

        // Monitor for another SIGINT, and force shutdown if received.
        tokio::spawn(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen for ctrl-c");

            warn!("Received another SIGINT, forcing shutdown");
            exit(1);
        });
    };

    let tcp_listener = TcpListener::bind(&args.listen_address)
        .await
        .with_context(|| format!("Failed to bind to address: {}", args.listen_address))?;

    let api = create_api();

    axum::serve(tcp_listener, api)
        .with_graceful_shutdown(api_shutdown)
        .await
        .map_err(|err| err.into())
}
