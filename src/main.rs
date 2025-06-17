mod cmd;
mod config;
mod entry;
mod util;

use std::path::PathBuf;
use std::{io, process};

use clap::{Command, CommandFactory as _, Parser, Subcommand};
use clap_complete::{Shell, generate};
use config::Config;
use owo_colors::{OwoColorize as _, colors};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// The root path for the persistence
    #[arg(long, value_name = "PATH", global = true)]
    root: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Persist a new path
    Persist { path: PathBuf },

    /// Prune the persistence paths
    Prune {},

    /// Show the status of the persistence
    Status {},

    /// Generate shell completion scripts
    #[command(hide = true)]
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = match cli.command {
        Commands::Completion { shell } => {
            let cmd: &mut Command = &mut Cli::command();
            generate(shell, cmd, cmd.get_name().to_string(), &mut io::stdout());
            Ok(())
        }
        _ => run_command(cli),
    } {
        eprintln!("{}: {err}", "Error".fg::<colors::Red>().bold());
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
