use serde::{Deserialize, Serialize};

use crate::entry;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub persistence: Vec<PersistenceInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersistenceInfo {
    pub root: entry::PersistEntry,
    pub directories: Vec<entry::PersistEntry>,
    pub files: Vec<entry::PersistEntry>,
}
