use std::fmt::Write;
use std::path::PathBuf;

use colored::Colorize;

pub fn print_delete_paths(root: &PathBuf, paths: &Vec<PathBuf>) -> String {
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
