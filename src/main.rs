mod command;
mod config;
mod entry;

use std::path::{Path, PathBuf};
use std::{fs, io};

use clap::{Parser, Subcommand};
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
    Add { path: PathBuf, root: PathBuf },

    /// Check the persistence paths and delete those that are not in the config
    Check {},
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    let config_path = cli
        .config
        .unwrap_or_else(|| PathBuf::from("/home/yufei/.config/ph/config.json"));

    let content = fs::read_to_string(config_path)?;

    let config: Config = serde_json::from_str(&content)?;

    match cli.command {
        Commands::Add { path, root } => {
            if path.exists() {
                eprintln!("Error: {} does not exist.", path.display());
            }
            let source_abs = fs::canonicalize(&path)?;

            let dest_dir = root.join(source_abs.strip_prefix("/").unwrap_or(&source_abs));

            let parent_dir = dest_dir.parent().unwrap();

            fs::create_dir_all(&parent_dir)?;

            if source_abs.is_dir() {
                copy_dir_recursive(&source_abs, &dest_dir)?;
            } else {
                fs::copy(&source_abs, &dest_dir)?;
            };

            println!("Persist {} to {}", source_abs.display(), dest_dir.display());
        }
        Commands::Check {} => {
            command::plan(&config)?;
        }
    }

    Ok(())
}
