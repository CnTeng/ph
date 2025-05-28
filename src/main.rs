mod config;
mod path_set;

use clap::{Parser, Subcommand};
use config::Config;
use path_set::PathSet;
use std::path::{Path, PathBuf};
use std::{fs, io};

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

fn check(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    fn collect_paths(
        dir: &std::path::Path,
        path_set: &PathSet,
        delete_paths: &mut Vec<PathBuf>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            match path_set.path.get(&path) {
                Some(&should_keep) => {
                    if !should_keep && path.is_dir() {
                        collect_paths(&path, path_set, delete_paths)?;
                    }
                }
                None => {
                    delete_paths.push(path);
                }
            }
        }
        Ok(())
    }

    let path_set = PathSet::from(&config.persistence[0]);
    let mut delete_paths = Vec::new();
    collect_paths(
        std::path::Path::new("/persist"),
        &path_set,
        &mut delete_paths,
    )?;

    println!("Paths to delete: {:#?}", delete_paths);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config_path = cli
        .config
        .unwrap_or_else(|| PathBuf::from("/etc/persist.json"));

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
        Commands::Check {} => check(&config)?,
    }

    Ok(())
}
