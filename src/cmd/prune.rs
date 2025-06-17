use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

use owo_colors::{OwoColorize as _, colors};
use tempfile::NamedTempFile;

use crate::cmd::format::format_entry_set;
use crate::config::PersistConfig;
use crate::entry::{PersistEntryMap, PersistEntrySet, find_deletable_entries};

pub fn prune(root: &Path, cfg: &PersistConfig) -> io::Result<()> {
    let mut entry_map = PersistEntryMap::from(root);
    entry_map.merge(&PersistEntryMap::from(cfg.directories.as_slice()));
    entry_map.merge(&PersistEntryMap::from(cfg.files.as_slice()));

    let deletable_entries = find_deletable_entries(root, &entry_map)?;
    if deletable_entries.is_empty() {
        println!("No paths to delete for root: {}", root.display());
        return Ok(());
    }

    let deletable_entries_str: Vec<&str> = deletable_entries
        .iter()
        .filter_map(|e| e.to_str())
        .collect();

    let kept_entries = create_temp_file(&deletable_entries_str)?;

    let kept_set: PersistEntrySet = kept_entries.into_iter().map(PathBuf::from).collect();
    let delete_set: PersistEntrySet = deletable_entries.difference(&kept_set).cloned().collect();

    println!(
        "Entries to delete for root: {}\n{}",
        root.display().bold(),
        format_entry_set(&delete_set)
    );
    confirm("Are you sure you want to delete these entries?")?;

    for entry in delete_set {
        if entry.is_dir() {
            std::fs::remove_dir_all(&entry)?;
        } else {
            std::fs::remove_file(&entry)?;
        }
        println!(
            "{}: Remove {}",
            "Finished".fg::<colors::Green>().bold(),
            entry.display()
        );
    }
    Ok(())
}

fn create_temp_file(entries: &[&str]) -> io::Result<Vec<String>> {
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

fn confirm(info: &str) -> io::Result<bool> {
    print!("{info} (y/n): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let answer = input.trim().to_ascii_lowercase();
    Ok(answer == "y" || answer == "yes")
}
