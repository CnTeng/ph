use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct PathSet {
    pub path: HashMap<PathBuf, bool>,
}

impl PathSet {
    pub fn new() -> Self {
        PathSet {
            path: HashMap::new(),
        }
    }

    pub fn merge(&mut self, other: &PathSet) {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct Path {
    pub path: PathBuf,
}

impl Path {
    pub fn ancestors_map(&self) -> PathSet {
        let mut ancestors = PathSet::new();

        for ancestor in self.path.ancestors() {
            if ancestor.as_os_str().is_empty() {
                continue;
            }
            ancestors.path.insert(ancestor.to_path_buf(), false);
        }

        ancestors.path.insert(self.path.clone(), true);
        ancestors
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct PathVec {
    pub path_set: Vec<Path>,
}

impl PathVec {
    pub fn ancestors_map(&self) -> PathSet {
        let mut ancestors = PathSet::new();
        self.path_set.iter().for_each(|path| {
            ancestors.merge(&path.ancestors_map());
        });
        ancestors
    }
}
