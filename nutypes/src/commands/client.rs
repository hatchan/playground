use anyhow::Result;
use clap::Subcommand;
use nutypes::models::ProductId;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,

    /// The base URL of the server.
    #[arg(
        short,
        long,
        env,
        global = true,
        default_value = "http://localhost:6767"
    )]
    pub base_url: url::Url,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    GetProduct(GetProductArgs),
}

pub async fn handle_command(args: Args) -> Result<()> {
    match args.command {
        Command::GetProduct(args) => handle_get_product_command(args).await,
    }
}

#[derive(Debug, clap::Args)]
pub struct GetProductArgs {
    product_id: ProductId,

    /// The base URL of the server.
    #[arg(from_global)]
    pub base_url: url::Url,
}

pub async fn handle_get_product_command(args: GetProductArgs) -> Result<()> {
    let client = nutypes::client::Client::builder().build(args.base_url);

    let result = client.get_product(&args.product_id).await;

    match result {
        Ok(product) => {
            println!("Product: {:?}", product.id);
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    };
    // TODO: Handle the result
    Ok(())
}
