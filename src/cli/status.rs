use std::collections::BTreeSet;
use std::fmt::Write;
use std::path::{Path, PathBuf};

use color_eyre::Result;
use colored::Colorize;

use crate::config::Config;
use crate::entry::{PersistEntrySet, check_delete_path};

pub fn status(config: &Config) -> Result<()> {
    for p in config.persistence.iter() {
        let mut path_set = PersistEntrySet::from(&p.root);
        path_set.merge(&PersistEntrySet::from(&p.directories));
        path_set.merge(&PersistEntrySet::from(&p.files));

        let delete_paths = check_delete_path(&p.root, &path_set)?;
        print!("{}", print_delete_paths(&p.root, &delete_paths));
    }

    Ok(())
}

pub fn print_delete_paths(root: &Path, paths: &BTreeSet<PathBuf>) -> String {
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
