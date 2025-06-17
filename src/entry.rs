use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::{fs, io};

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

impl From<&Path> for PersistEntryMap {
    fn from(entry_path: &Path) -> Self {
        let mut set = Self::new();
        for ancestor in entry_path.ancestors() {
            if ancestor.as_os_str().is_empty() {
                continue;
            }
            set.entries.insert(ancestor.to_path_buf(), false);
        }
        set.entries.insert(entry_path.to_path_buf(), true);
        set
    }
}

impl From<&[PathBuf]> for PersistEntryMap {
    fn from(entry_vec: &[PathBuf]) -> Self {
        let mut set = PersistEntryMap::new();
        entry_vec.iter().for_each(|path| {
            set.merge(&PersistEntryMap::from(path.as_path()));
        });
        set
    }
}

pub fn find_deletable_entries(
    root: &Path,
    entry_map: &PersistEntryMap,
) -> io::Result<PersistEntrySet> {
    let mut delete_paths = BTreeSet::new();
    collect_deletable_entries(root, entry_map, &mut delete_paths)?;
    Ok(delete_paths)
}

fn collect_deletable_entries(
    dir: &Path,
    entry_map: &PersistEntryMap,
    entry_set: &mut PersistEntrySet,
) -> io::Result<()> {
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

pub fn persist_entry(root: &Path, entry: &Path) -> io::Result<PathBuf> {
    let src = fs::canonicalize(entry)?;
    let dst = root.join(src.strip_prefix("/").unwrap_or(&src));

    if dst.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Destination path already exists: {}", dst.display()),
        ));
    }

    for src_ancestor in src.ancestors() {
        let meta = fs::metadata(src_ancestor)?;
        let dst_ancestor = root.join(src_ancestor.strip_prefix("/").unwrap_or(src_ancestor));
        if dst_ancestor.is_dir() {
            util::create_dir_with_metadata(&dst_ancestor, &meta)?;
        }
    }

    if src.is_dir() {
        util::copy_dir_recursive_with_metadata(&src, &dst)?;
    } else {
        util::copy_file_with_metadata(&src, &dst)?;
    };

    Ok(dst)
}
