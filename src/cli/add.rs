use std::io;
use std::path::Path;

use crate::entry::{persist_entry, PersistEntry};

pub fn add(path: &Path, root: &Path) -> io::Result<()> {
    return persist_entry(&PersistEntry::new(path.to_path_buf()), root);
}
