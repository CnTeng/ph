mod cmd;
mod config;
mod entry;
mod util;

use std::path::PathBuf;
use std::{io, process};

use clap::{CommandFactory as _, Parser, Subcommand};
use clap_complete::{Shell, generate};
use config::Config;
use owo_colors::{OwoColorize as _, colors};

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
    #[command(hide = true)]
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Completion { shell } => {
            generate(shell, &mut Cli::command(), "ph", &mut io::stdout());
            Ok(())
        }
        _ => run_command(cli),
    };

    if let Err(e) = &result {
        eprintln!("{}: {e}", "Error".fg::<colors::Red>().bold());
        process::exit(1);
    }
}

fn run_command(cli: Cli) -> io::Result<()> {
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
