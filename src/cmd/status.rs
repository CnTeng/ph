use std::fmt::Write;
use std::io;
use std::path::Path;

use owo_colors::OwoColorize as _;

use crate::config::PersistConfig;
use crate::entry::{PersistEntryMap, PersistEntrySet, find_deletable_entries};

pub fn status(root: &Path, cfg: &PersistConfig) -> io::Result<()> {
    let mut path_set = PersistEntryMap::from(root);
    path_set.merge(&PersistEntryMap::from(cfg.directories.as_slice()));
    path_set.merge(&PersistEntryMap::from(cfg.files.as_slice()));

    let delete_paths = find_deletable_entries(root, &path_set)?;
    print!("{}", print_delete_paths(root, &delete_paths));

    Ok(())
}

fn print_delete_paths(root: &Path, paths: &PersistEntrySet) -> String {
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
        paths.iter().for_each(|path| {
            writeln!(output, "\t{}", path.display().to_string().red()).unwrap();
        });
        output
    }
}
