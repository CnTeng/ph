use crate::config::PersistenceInfo;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PathSet {
    pub path: HashMap<PathBuf, bool>,
}

impl From<&PersistenceInfo> for PathSet {
    fn from(info: &PersistenceInfo) -> Self {
        let mut path_set = PathSet {
            path: HashMap::new(),
        };

        path_set.path.insert(info.root.clone(), false);

        for directory in &info.directories {
            for ancestor in directory.ancestors() {
                if ancestor.as_os_str().is_empty() {
                    continue;
                }

                path_set.path.insert(ancestor.to_path_buf(), false);
            }

            path_set.path.insert(directory.to_path_buf(), true);
        }

        for file in &info.files {
            for ancestor in file.ancestors() {
                if ancestor.as_os_str().is_empty() {
                    continue;
                }

                path_set.path.insert(ancestor.to_path_buf(), false);
            }

            path_set.path.insert(file.to_path_buf(), true);
        }

        path_set
    }
}
