use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::{fs, io};

use color_eyre::Result;

use crate::util;

pub fn persist_entry(entry: &PathBuf, root: &Path) -> Result<()> {
    if !entry.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Path does not exist: {}", entry.display()),
        )
        .into());
    }

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

impl From<&PathBuf> for PersistEntrySet {
    fn from(persist_path: &PathBuf) -> Self {
        let mut set = PersistEntrySet::new();

        for ancestor in persist_path.ancestors() {
            if ancestor.as_os_str().is_empty() {
                continue;
            }
            set.path.insert(ancestor.to_path_buf(), false);
        }

        set.path.insert(persist_path.clone(), true);
        set
    }
}

impl From<&Vec<PathBuf>> for PersistEntrySet {
    fn from(vec: &Vec<PathBuf>) -> Self {
        let mut set = PersistEntrySet::new();
        vec.iter().for_each(|path| {
            set.merge(&PersistEntrySet::from(path));
        });
        set
    }
}

pub fn check_delete_path(root: &PathBuf, path_set: &PersistEntrySet) -> Result<BTreeSet<PathBuf>> {
    let mut delete_paths = BTreeSet::new();
    util::collect_paths(&root, &path_set, &mut delete_paths)?;

    Ok(delete_paths)
}
