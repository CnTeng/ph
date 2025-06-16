use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use color_eyre::Result;
use tempfile::NamedTempFile;

use crate::config::Config;
use crate::entry::{PersistEntryMap, PersistEntrySet, find_deletable_entries};

pub fn prune(config: &Config) -> Result<()> {
    for p in config.persistence.iter() {
        let mut entry_map = PersistEntryMap::from(&p.root);
        entry_map.merge(&PersistEntryMap::from(p.directories.as_slice()));
        entry_map.merge(&PersistEntryMap::from(p.files.as_slice()));

        let deletable_entries = find_deletable_entries(&p.root, &entry_map)?;
        if deletable_entries.is_empty() {
            println!("No paths to delete for root: {}", p.root.display());
        }

        let deletable_entries_str: Vec<&str> = deletable_entries
            .iter()
            .filter_map(|e| e.to_str())
            .collect();

        let edited_entries = create_temp_file(&deletable_entries_str)?;

        let edited_set: PersistEntrySet = edited_entries.into_iter().map(PathBuf::from).collect();
        let delete_set: PersistEntrySet =
            deletable_entries.difference(&edited_set).cloned().collect();

        for entry in delete_set {
            match fs::metadata(&entry) {
                Ok(meta) => {
                    if meta.is_dir() {
                        std::fs::remove_dir_all(&entry)?;
                    } else {
                        std::fs::remove_file(&entry)?;
                    }
                    println!("Deleted: {}", entry.display());
                }
                Err(_) => {
                    println!(
                        "Path does not exist, skipping deletion: {}",
                        entry.display()
                    );
                }
            }
        }
    }
    Ok(())
}

fn create_temp_file(entries: &[&str]) -> Result<Vec<String>> {
    let mut temp = NamedTempFile::new()?;
    for line in entries {
        writeln!(temp, "{line}")?;
    }

    let path = temp.path().to_str().unwrap();
    let editor = std::env::var("EDITOR").unwrap_or("vi".into());
    Command::new(editor).arg(path).status()?;

    let edited: Vec<String> = fs::read_to_string(path)?
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .collect();

    Ok(edited)
}
