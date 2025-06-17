mod cmd;
mod config;
mod entry;
mod util;

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
	let config = Config::load()?;
	let (root, cfg) = config.get_persist_config(cli.root.as_deref())?;

	match cli.command {
		Commands::Persist { path } => cmd::persist(&root, &path)?,
		Commands::Prune {} => cmd::prune(&root, cfg)?,
		Commands::Status {} => cmd::status(&root, cfg)?,
	}

	Ok(())
}
