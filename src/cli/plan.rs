use std::io;
use std::path::PathBuf;

use crate::config::Config;
use crate::entry::{PersistEntrySet, check_delete_path};

pub fn plan(config: &Config) -> Result<Vec<PathBuf>, io::Error> {
    let mut output: Vec<PathBuf> = Vec::new();
    for p in config.persistence.iter() {
        let mut path_set = PersistEntrySet::from(&p.root);
        path_set.merge(&PersistEntrySet::from(&p.directories));
        path_set.merge(&PersistEntrySet::from(&p.files));

        let delete_paths = check_delete_path(&p.root, &path_set)?;
        output.extend(delete_paths.clone());
    }

    Ok(output)
}
