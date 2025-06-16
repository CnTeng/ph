use std::fs::read_to_string;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use color_eyre::Result;
use tempfile::NamedTempFile;

use crate::config::Config;
use crate::entry::{PersistEntryMap, PersistEntrySet, find_deletable_entries};

pub fn prune(config: &Config) -> Result<()> {
    for p in config.persistence.iter() {
        let mut path_set = PersistEntryMap::from(&p.root);
        path_set.merge(&PersistEntryMap::from(p.directories.as_slice()));
        path_set.merge(&PersistEntryMap::from(p.files.as_slice()));

        let delete_paths = find_deletable_entries(&p.root, &path_set)?;
        if delete_paths.is_empty() {
            println!("No paths to delete for root: {}", p.root.display());
        }

        let delete_paths_str: Vec<String> = delete_paths
            .iter()
            .map(|path| path.display().to_string())
            .collect();
        let edited = create_temp_file(delete_paths_str.iter().map(String::as_str).collect())?;

        let edit_set: PersistEntrySet = edited.into_iter().map(PathBuf::from).collect();
        let diff: PersistEntrySet = delete_paths.difference(&edit_set).cloned().collect();
        for path in diff {
            if path.exists() {
                println!("Deleted: {}", path.display());
            }
        }
    }
    Ok(())
}

fn create_temp_file(data: Vec<&str>) -> Result<Vec<String>> {
    let mut temp = NamedTempFile::new()?;
    for line in data {
        writeln!(temp, "{line}")?;
    }

    let path = temp.path().to_str().unwrap();
    let editor = std::env::var("EDITOR").unwrap_or("vi".into());
    Command::new(editor).arg(path).status()?;

    let edited = read_to_string(path)?;
    let edited: Vec<String> = edited
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(String::from)
        .collect();
    Ok(edited)
}
