use std::path::Path;

use color_eyre::Result;

use crate::entry::{PersistEntry, persist_entry};

pub fn add(path: &Path, root: &Path) -> Result<()> {
    return persist_entry(&PersistEntry::new(path.to_path_buf()), root);
}
