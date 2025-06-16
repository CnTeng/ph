use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use color_eyre::Result;

use crate::util;

pub type PersistEntrySet = BTreeSet<PathBuf>;

#[derive(Debug)]
pub struct PersistEntryMap {
    pub entries: HashMap<PathBuf, bool>,
}

impl PersistEntryMap {
    pub fn new() -> Self {
        PersistEntryMap {
            entries: HashMap::new(),
        }
    }

    pub fn merge(&mut self, other: &PersistEntryMap) {
        for (entry, keep) in &other.entries {
            self.entries
                .entry(entry.clone())
                .and_modify(|e| *e |= *keep)
                .or_insert(*keep);
        }
    }
}

impl From<&PathBuf> for PersistEntryMap {
    fn from(entry_path: &PathBuf) -> Self {
        let mut set = Self::new();
        for ancestor in entry_path.ancestors() {
            if ancestor.as_os_str().is_empty() {
                continue;
            }
            set.entries.insert(ancestor.to_path_buf(), false);
        }
        set.entries.insert(entry_path.clone(), true);
        set
    }
}

impl From<&[PathBuf]> for PersistEntryMap {
    fn from(entry_vec: &[PathBuf]) -> Self {
        let mut set = PersistEntryMap::new();
        entry_vec.iter().for_each(|path| {
            set.merge(&PersistEntryMap::from(path));
        });
        set
    }
}

pub fn find_deletable_entries(root: &Path, path_set: &PersistEntryMap) -> Result<PersistEntrySet> {
    let mut delete_paths = BTreeSet::new();
    collect_deletable_entries(&root, &path_set, &mut delete_paths)?;
    Ok(delete_paths)
}

fn collect_deletable_entries(
    dir: &Path,
    entry_map: &PersistEntryMap,
    entry_set: &mut PersistEntrySet,
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        match entry_map.entries.get(&path) {
            Some(true) => continue, // This path is kept, skip it
            Some(false) => {
                if path.is_dir() {
                    collect_deletable_entries(&path, entry_map, entry_set)?;
                }
            }
            None => {
                entry_set.insert(path);
            }
        }
    }
    Ok(())
}

pub fn persist_entry(entry: &Path, root: &Path) -> Result<()> {
    let source_abs = fs::canonicalize(&entry)?;
    let dest_dir = root.join(source_abs.strip_prefix("/").unwrap_or(&source_abs));
    let parent_dir = dest_dir.parent().unwrap();

    fs::create_dir_all(&parent_dir)?;

    if source_abs.is_dir() {
        util::copy_dir_recursive(&source_abs, &dest_dir)?;
    } else {
        util::copy_file_with_owner(&source_abs, &dest_dir)?;
    };

    Ok(())
}
