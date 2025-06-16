use std::fmt::Write;
use std::path::Path;

use color_eyre::Result;
use colored::Colorize;

use crate::config::Config;
use crate::entry::{PersistEntryMap, PersistEntrySet, find_deletable_entries};

pub fn status(config: &Config) -> Result<()> {
    for p in config.persistence.iter() {
        let mut path_set = PersistEntryMap::from(&p.root);
        path_set.merge(&PersistEntryMap::from(p.directories.as_slice()));
        path_set.merge(&PersistEntryMap::from(p.files.as_slice()));

        let delete_paths = find_deletable_entries(&p.root, &path_set)?;
        print!("{}", print_delete_paths(&p.root, &delete_paths));
    }

    Ok(())
}

pub fn print_delete_paths(root: &Path, paths: &PersistEntrySet) -> String {
    if paths.is_empty() {
        format!(
            "No paths to delete for root: {}",
            root.display().to_string().bold()
        )
    } else {
        let mut output = String::new();
        writeln!(
            output,
            "Paths to delete for root: {}",
            root.display().to_string().bold()
        )
        .unwrap();
        paths.into_iter().for_each(|path| {
            writeln!(output, "\t{}", path.display().to_string().red()).unwrap();
        });
        output
    }
}
