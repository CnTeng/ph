use std::path::Path;

use color_eyre::Result;

use crate::entry::persist_entry;

pub fn add(path: &Path, root: &Path) -> Result<()> {
    return persist_entry(&path.to_path_buf(), root);
}
