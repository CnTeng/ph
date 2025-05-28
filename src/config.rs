use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub persistence: Vec<PersistenceInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersistenceInfo {
    pub enable: bool,
    pub root: PathBuf,
    pub directories: Vec<PathBuf>,
    pub files: Vec<PathBuf>,
}
