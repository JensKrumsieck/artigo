use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub  enum Commands {
    #[command(about = "Starts the server")]
    Serve,
    #[command(about = "Migrates the database")]
    Migrate,
}
