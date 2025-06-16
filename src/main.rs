mod cmd;
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
    root: Option<PathBuf>,

    #[arg(short, long, value_name = "FILE", global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a path to the persistence configuration
    Persist { path: PathBuf },

    /// Prune the persistence paths by deleting those that are not in the config
    Prune {},

    /// Check the persistence paths and delete those that are not in the config
    Status {},
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let config_path = cli
        .config
        .unwrap_or_else(|| PathBuf::from("/home/yufei/.config/ph/config.json"));

    let content = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&content)?;

    let (root, cfg) = if config.persistence.len() == 1 {
        config.persistence.iter().next().unwrap()
    } else if let Some(root) = &cli.root {
        let cfg = config.persistence.get(root).ok_or_else(|| {
            color_eyre::eyre::eyre!("Root path not found in config: {}", root.display())
        })?;
        (root, cfg)
    } else {
        return Err(color_eyre::eyre::eyre!(
            "Multiple persistence paths found, please specify one using --root"
        ));
    };

    match cli.command {
        Commands::Persist { path } => cmd::persist(root, &path)?,
        Commands::Prune {} => cmd::prune(root, cfg)?,
        Commands::Status {} => cmd::status(root, cfg)?,
    }

    Ok(())
}
