use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub persistence: Vec<PersistConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersistConfig {
    pub root: PathBuf,
    pub directories: Vec<PathBuf>,
    pub files: Vec<PathBuf>,
}
