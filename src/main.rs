use artigo::{
    cli::{Cli, Commands},
    server,
};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.commands {
        Commands::Serve => server::serve().await?,
        Commands::Migrate => server::migrate().await?,
    }
    Ok(())
}