use crate::path;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub persistence: Vec<PersistenceInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersistenceInfo {
    pub root: path::Path,
    pub directories: path::PathVec,
    pub files: path::PathVec,
}

impl PersistenceInfo {
    fn ancestors_map(&self) -> path::PathSet {
        let mut ancestors = self.root.ancestors_map();
        ancestors.merge(&self.directories.ancestors_map());
        ancestors.merge(&self.files.ancestors_map());
        ancestors
    }

    pub fn check(&self) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let path_set = self.ancestors_map();
        let mut delete_paths = Vec::new();
        collect_paths(
            std::path::Path::new("/persist"),
            &path_set,
            &mut delete_paths,
        )?;

        Ok(delete_paths)
    }
}

fn collect_paths(
    dir: &std::path::Path,
    path_set: &path::PathSet,
    delete_paths: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        match path_set.path.get(&path) {
            Some(&is_keep) => {
                if !is_keep && path.is_dir() {
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
