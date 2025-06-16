use std::path::Path;

use color_eyre::Result;

use crate::entry::persist_entry;

pub fn persist(root: &Path, entry: &Path) -> Result<()> {
    return persist_entry(root, entry);
}
