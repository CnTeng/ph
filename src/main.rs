mod cli;
mod config;
mod entry;
mod util;

use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::Result;
use config::Config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, value_name = "FILE", global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a path to the persistence configuration
    Add {
        path: PathBuf,
        root: PathBuf,
    },

    /// Check the persistence paths and delete those that are not in the config
    Status {},

    Prune {},
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let config_path = cli
        .config
        .unwrap_or_else(|| PathBuf::from("/home/yufei/.config/ph/config.json"));

    let content = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&content)?;

    match cli.command {
        Commands::Add { path, root } => cli::add(&path, &root)?,
        Commands::Status {} => {
            cli::status(&config)?;
        }
        Commands::Prune {} => {
            cli::prune(&config)?;
        }
    }

    Ok(())
}
