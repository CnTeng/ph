use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct PersistEntry {
    pub path: PathBuf,
}

pub struct PersistEntrySet {
    pub path: HashMap<PathBuf, bool>,
}

impl PersistEntrySet {
    pub fn new() -> Self {
        PersistEntrySet {
            path: HashMap::new(),
        }
    }

    pub fn merge(&mut self, other: &PersistEntrySet) {
        other.path.iter().for_each(|(path, recursive)| {
            self.path
                .entry(path.clone())
                .and_modify(|e| {
                    *e = *e || *recursive;
                })
                .or_insert(*recursive);
        });
    }
}

impl From<&PersistEntry> for PersistEntrySet {
    fn from(persist_path: &PersistEntry) -> Self {
        let mut set = PersistEntrySet::new();

        for ancestor in persist_path.path.ancestors() {
            if ancestor.as_os_str().is_empty() {
                continue;
            }
            set.path.insert(ancestor.to_path_buf(), false);
        }

        set.path.insert(persist_path.path.clone(), true);
        set
    }
}

impl From<&Vec<PersistEntry>> for PersistEntrySet {
    fn from(vec: &Vec<PersistEntry>) -> Self {
        let mut set = PersistEntrySet::new();
        vec.iter().for_each(|path| {
            set.merge(&PersistEntrySet::from(path));
        });
        set
    }
}

fn collect_paths(
    dir: &Path,
    path_set: &PersistEntrySet,
    delete_paths: &mut Vec<PathBuf>,
) -> Result<(), io::Error> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        match path_set.path.get(&path) {
            Some(true) => continue, // This path is kept, skip it
            Some(false) => {
                if path.is_dir() {
                    collect_paths(&path, path_set, delete_paths)?;
                }
            }
            None => delete_paths.push(path),
        }
    }
    Ok(())
}

pub fn check_delete_path(
    root: &PersistEntry,
    path_set: &PersistEntrySet,
) -> Result<Vec<PathBuf>, io::Error> {
    let mut delete_paths = Vec::new();
    collect_paths(&root.path, &path_set, &mut delete_paths)?;

    Ok(delete_paths)
}
