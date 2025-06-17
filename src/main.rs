mod cmd;
mod config;
mod entry;
mod util;

use std::io;
use std::path::PathBuf;

use clap::{CommandFactory as _, Parser, Subcommand};
use clap_complete::{Shell, generate};
use color_eyre::Result;
use config::Config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,

	/// The root path for the persistence configuration
	#[arg(long, value_name = "PATH", global = true)]
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

	/// Generate completion scripts for the CLI
	Completion {
		#[arg(value_enum)]
		shell: Shell,
	},
}

fn main() -> Result<()> {
	color_eyre::install()?;

	let cli = Cli::parse();
	match cli.command {
		Commands::Completion { shell } => {
			generate(shell, &mut Cli::command(), "ph", &mut io::stdout());
		}
		_ => run_command(cli)?,
	}

	Ok(())
}

fn run_command(cli: Cli) -> Result<()> {
	let config = Config::load()?;
	let (root, cfg) = config.get_persist_config(cli.root.as_deref())?;

	match cli.command {
		Commands::Persist { path } => cmd::persist(&root, &path)?,
		Commands::Prune {} => cmd::prune(&root, cfg)?,
		Commands::Status {} => cmd::status(&root, cfg)?,
		Commands::Completion { .. } => unreachable!(),
	}

	Ok(())
}
