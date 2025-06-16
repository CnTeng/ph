use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub persistence: Vec<PersistenceInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersistenceInfo {
    pub root: PathBuf,
    pub directories: Vec<PathBuf>,
    pub files: Vec<PathBuf>,
}
