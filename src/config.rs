use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub persistence: HashMap<PathBuf, PersistConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PersistConfig {
    pub directories: Vec<PathBuf>,
    pub files: Vec<PathBuf>,
}
