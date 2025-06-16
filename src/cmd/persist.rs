use std::path::Path;

use color_eyre::Result;

use crate::entry::persist_entry;

pub fn persist(root: &Path, entry: &Path) -> Result<()> {
    let dst = persist_entry(root, entry)?;
    println!("Persisted entry: {} to {}", entry.display(), dst.display());

    Ok(())
}
