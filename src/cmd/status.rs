use std::io;
use std::path::Path;

use owo_colors::OwoColorize as _;

use crate::cmd::format::format_entry_set;
use crate::config::PersistConfig;
use crate::entry::{PersistEntryMap, find_deletable_entries};

pub fn status(root: &Path, cfg: &PersistConfig) -> io::Result<()> {
    let mut entry_map = PersistEntryMap::from(root);
    entry_map.merge(&PersistEntryMap::from(cfg.directories.as_slice()));
    entry_map.merge(&PersistEntryMap::from(cfg.files.as_slice()));

    let deletable_entries = find_deletable_entries(root, &entry_map)?;
    if deletable_entries.is_empty() {
        println!("No paths to delete for root: {}", root.display().bold());
        return Ok(());
    }

    println!(
        "No paths to delete for root: {}\n{}",
        root.display().bold(),
        format_entry_set(&deletable_entries)
    );

    Ok(())
}
