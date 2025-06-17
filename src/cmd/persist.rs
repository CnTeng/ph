use std::path::Path;
use std::{fs, io};

use owo_colors::{OwoColorize as _, colors};

use crate::cmd::format::format_metadata;
use crate::entry::persist_entry;

pub fn persist(root: &Path, entry: &Path) -> io::Result<()> {
    let dst = persist_entry(root, entry)?;

    println!(
        "{}: Persist {} to {}",
        "Finished".fg::<colors::Green>().bold(),
        entry.display().bold(),
        dst.display().bold()
    );

    println!("Source:");
    print_metadata("target", entry)?;
    if let Some(src_parent) = entry.parent() {
        print_metadata("parent", src_parent)?;
    }

    println!("Destination:");
    print_metadata("target", &dst)?;
    if let Some(dst_parent) = dst.parent() {
        print_metadata("parent", dst_parent)?;
    }

    Ok(())
}

fn print_metadata(info: &str, path: &Path) -> io::Result<()> {
    let meta = fs::metadata(path)?;

    println!(
        "\t{}: {} {}",
        info,
        format_metadata(&meta),
        path.display().bold()
    );

    Ok(())
}
